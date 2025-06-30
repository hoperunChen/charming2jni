use std::path::Path;

use charming::{ImageFormat, ImageRenderer};
use jni::JNIEnv;
use jni::objects::*;
use jni::sys::jint;
use thiserror::Error;

macro_rules! unwarp_exception {
    ($val:expr,$env:expr,$err_val:expr) => {
        match $val {
            Ok(v) => v,
            Err(e) => {
                $env.exception_clear().expect("clear");
                $env.throw_new("java/lang/RuntimeException", format!("{}", e))
                    .unwrap();
                return $err_val;
            }
        }
    };
}

#[derive(Error, Debug)]
pub enum EchartsError {
    #[error("文件格式错误")]
    Format,
    #[error("未知错误，请提交issue")]
    Unknown,
}

/// 跟进路径和原始json渲染图表
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_top_magicpotato_Echarts_save(
    mut env: JNIEnv,
    _class: JClass,
    width: jint,
    height: jint,
    path: JString,
    data: JString,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let data: String = env
            .get_string(&data)
            .expect("Couldn't get data string!")
            .into();
        let path: String = env
            .get_string(&path)
            .expect("Couldn't get path string!")
            .into();
        let mut renderer = ImageRenderer::new(width as u32, height as u32);

        let path = Path::new(&path);
        if path.extension().expect("无法获取文件扩展名") == "svg" {
            unwarp_exception!(renderer.save_by_json(data, path), env, ());
        } else {
            let extension = unwarp_exception!(parse_extension_by_path(path), env, ());
            unwarp_exception!(renderer.save_format_by_json(extension, data, path), env, ());
        }
    }));
    if let Err(_) = result {
        let _ = env.exception_clear();
        let _ = env.throw_new("java/lang/RuntimeException", "Rust panic: native code panicked");
        return;
    }
}

/// 跟进json渲染数据  返回byte数组
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_top_magicpotato_Echarts_render<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    width: jint,
    height: jint,
    extension: JString<'local>,
    data: JString<'local>,
) -> JByteArray<'local> {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let data: String = env
            .get_string(&data)
            .expect("Couldn't get data string!")
            .into();
        let extension: String = env
            .get_string(&extension)
            .expect("Couldn't get extension string!")
            .into();
        let mut renderer = ImageRenderer::new(width as u32, height as u32);

        if extension == "svg" {
            let x = unwarp_exception!(renderer.render_by_json(data), env, JByteArray::default());
            env.byte_array_from_slice(x.as_bytes()).unwrap()
        } else {
            let extension = unwarp_exception!(parse_extension(&extension), env, JByteArray::default());
            let x = unwarp_exception!(
                renderer.render_format_by_json(extension, data),
                env,
                JByteArray::default()
            );
            env.byte_array_from_slice(x.as_slice()).unwrap()
        }
    }));
    match result {
        Ok(arr) => arr,
        Err(_) => {
            let _ = env.exception_clear();
            let _ = env.throw_new("java/lang/RuntimeException", "Rust panic: native code panicked");
            JByteArray::default()
        }
    }
}

/// 通过文件名字解析扩展名
fn parse_extension_by_path(path: &Path) -> Result<ImageFormat, EchartsError> {
    ImageFormat::from_path(path).map_err(|_| EchartsError::Format)
}

fn parse_extension(ext: &str) -> Result<ImageFormat, EchartsError> {
    ImageFormat::from_extension(ext).ok_or(EchartsError::Format)
}
