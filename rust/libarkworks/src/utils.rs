// use jni::{sys::jbyteArray, JNIEnv, objects::JByteArray};


// pub fn to_jbyte_array<'local>(env: &JNIEnv<'local>, buf: &mut [u8]) -> jbyteArray {
//     let output = env.new_byte_array(buf.len() as i32).expect("failed to create java byte array");
//     env.set_byte_array_region(&output, 0, bytemuck::cast_slice(buf)).unwrap();
//     output.into_raw()
// }

// pub fn to_u8_vec<'local>(env: &JNIEnv<'local>, arr: &JByteArray<'local>) -> Vec<u8> {
//     let size = env.get_array_length(arr).unwrap().try_into().unwrap();
//     let mut buf = vec![0; size];
//     env.get_byte_array_region(arr, 0, &mut buf).expect("failed to read java byte array");
//     Vec::from(bytemuck::cast_slice(&buf))
// }