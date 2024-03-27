//! # jjss_tocargo
//! 
//! `jjss_tocargo` is a test cargo.


pub mod utils;
pub mod location;


/// The version of the crate
const VERSION : &str = "0.1.2";

/// The maximum number of tries
const MAX_TRY : u32 = 10;

/// Adds two numbers together.
/// 
/// # Examples
/// ```
/// let sum = jjss_tocargo::add(1, 2);
/// println!("{}", sum);
/// ```
/// 
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Greeting function
/// 
/// 인사말을 출력해주는 함수입니다.
/// documentation에 한글이 출력될까요?
/// 
/// 
/// # Examples
/// - example에 대한 설명입니다.
///     - 샵 두개가 될까요?
/// 
/// - example 1
/// ```
/// jjss_tocargo::test_function();
/// ```
/// -example 2
/// ```
/// jjss_tocargo::test_function();
/// ```
pub fn test_function() {
    println!("Hello world! version 0.1.2");
}


/// 코멘트 확인용 함수
/// 자주 사용되는 구문은
/// # Examples
/// - 활용 예제
/// # Panics
/// - 패닉을 일으킬 수 있는 시나리오
/// # Errors
/// - Result를 반환할 경우 발생할 수 있는 에러의 종류와 에러 발생 조건을 설명
/// # Safety
/// -함수가 안전하지 않을(unsafe) 경우
pub async fn commement_test_func() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}