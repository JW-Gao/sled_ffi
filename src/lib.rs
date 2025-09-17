use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::slice;

// 定义C兼容的枚举，用于返回结果
#[repr(C)]
pub enum SledResult {
    Success,
    Error,
}

// 定义C兼容的数据结构，用于返回数据
#[repr(C)]
pub struct SledData {
    pub ptr: *const u8,
    pub len: usize,
}

// 将 sled::Db 类型包装在一个不透明的结构体中
pub struct SledDb(sled::Db);

// --- FFI 函数 ---

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sled_open(path: *const c_char) -> *mut SledDb {
    unsafe {
        if path.is_null() {
            return ptr::null_mut();
        }
        let c_str = CStr::from_ptr(path);
        let path_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        };

        match sled::open(path_str) {
            Ok(db) => Box::into_raw(Box::new(SledDb(db))),
            Err(_) => ptr::null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sled_close(db_ptr: *mut SledDb) {
    unsafe {
        if !db_ptr.is_null() {
            let _ = Box::from_raw(db_ptr);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sled_insert(
    db_ptr: *mut SledDb,
    key_ptr: *const u8,
    key_len: usize,
    val_ptr: *const u8,
    val_len: usize,
) -> SledResult {
    unsafe {
        let db = &(*db_ptr).0; // 解引用裸指针是不安全操作
        let key = slice::from_raw_parts(key_ptr, key_len); // from_raw_parts是不安全操作
        let value = slice::from_raw_parts(val_ptr, val_len);

        match db.insert(key, value) {
            Ok(_) => SledResult::Success,
            Err(_) => SledResult::Error,
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sled_get(
    db_ptr: *mut SledDb,
    key_ptr: *const u8,
    key_len: usize,
) -> SledData {
    unsafe {
        let db = &(*db_ptr).0; // 解引用裸指针
        let key = slice::from_raw_parts(key_ptr, key_len); // from_raw_parts

        match db.get(key) {
            Ok(Some(ivec)) => {
                let mut owned_vec = ivec.to_vec();
                let ptr = owned_vec.as_mut_ptr();
                let len = owned_vec.len();
                std::mem::forget(owned_vec);
                SledData { ptr, len }
            }
            _ => SledData {
                ptr: ptr::null(),
                len: 0,
            },
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sled_free_data(data: SledData) {
    unsafe {
        if !data.ptr.is_null() {
            // from_raw_parts是不安全操作
            let _ = Vec::from_raw_parts(data.ptr as *mut u8, data.len, data.len);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sled_remove(
    db_ptr: *mut SledDb,
    key_ptr: *const u8,
    key_len: usize,
) -> SledResult {
    unsafe {
        let db = &(*db_ptr).0; // 解引用裸指针
        let key = slice::from_raw_parts(key_ptr, key_len); // from_raw_parts

        match db.remove(key) {
            Ok(_) => SledResult::Success,
            Err(_) => SledResult::Error,
        }
    }
}