use std::str;

// 实现一个存储结构清晰的evm栈结构，存储时尽量不使用第三方库的类型比如BigUint进行存储
// 但计算时可以借用BigUint进行计算，避免复杂的位运算
#[derive(Debug)]
struct EvmStack {
    stack: Vec<StackData>,
}

#[derive(Debug)]
struct StackData {
    // 栈宽存储的位数256位，最大深度1024
    // 16*16 = 256 使用u16，数组长度为16。
    // 8 *32 = 256 使用u8，数组长度为8，
    // 0xff 一个十六进制数f,只有4位。 ff 1111 1111 是8位
    data: Vec<u8>,
    // 符号位，0表示正数，1表示负数
    sign: u8,
}
impl StackData {
    fn new(bytes: Vec<u8>) -> Self {
        if bytes.len() > 32 {
            panic!("stack overflow");
        }
        println!("bytes: {:?},len:{:?}", bytes, bytes.len());
        let mut data = [0u8; 32].to_vec();
        // bytes 长度小于32时，则前补0填充
        // 补0策略1：data低字节不断插入bytes的元素，截取长度32-len..32
        if bytes.len() <= 32 {
            data = [[0u8; 32].to_vec(), bytes.clone()].concat();
        }
        println!("data: {:?}", data);
        Self {
            data: data[bytes.len()..].to_vec(),
            sign: 0,
        }
    }
}
impl EvmStack {
    fn new() -> Self {
        Self {
            stack: Vec::<StackData>::new(),
        }
    }
    fn push(&mut self, value: Vec<u8>) {
        if self.stack.len() > 1024 {
            panic!("stack overflow");
        }
        self.stack.push(StackData::new(value));
    }
    fn pop(&mut self) {
        if self.stack.is_empty() {
            panic!("stack underflow");
        }
        self.stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use super::*;

    #[test]
    fn test_evm_stack() {
        let excute_codes = "60016011600291";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_stack = EvmStack::new();
        // bytes 入栈 stack
        for byte in bytes {
            println!("byte: {:?}", byte);
            evm_stack.push(byte.to_be_bytes().to_vec());
        }
        println!("stack: {:?}", evm_stack.stack);
    }

    #[test]
    fn test_stack_data() {
        //0xff 1111 1111 255
        let stack_data: StackData = StackData::new(
            ((BigUint::from(1u8) << 256usize) - BigUint::from(1u8))
                .to_bytes_be()
                .to_vec(),
        );
        println!("stack_data: {:?}", stack_data);
        println!("data len : {:?}", stack_data.data.len());
        println!("data: {:?}", hex::encode(stack_data.data));
    }
}
