pub fn square(value: u32) -> u64 {
    u64::from(value) * u64::from(value)
}

/// Указатель на функцию имеет обычный именуемый тип и не хранит состояние.
pub fn apply_function(function: fn(u32) -> u64, value: u32) -> u64 {
    function(value)
}

/// Принимает замыкание или функцию, которую можно вызывать многократно.
pub fn call_twice<F>(function: F, value: u32) -> (u32, u32)
where
    F: Fn(u32) -> u32,
{
    (function(value), function(value))
}

/// `impl FnMut` позволяет замыканию изменять захваченное состояние.
pub fn accumulate_with(mut function: impl FnMut(u32), values: &[u32]) {
    for value in values {
        function(*value);
    }
}

/// Возвращает замыкание, владеющее изменяемым состоянием `x`.
pub fn get_adder(mut x: u32) -> impl FnMut(u32) -> u32 {
    move |a| {
        x += a;
        x
    }
}

/// `FnOnce` позволяет поглотить захваченное значение при вызове.
pub fn consume_once<F>(function: F) -> String
where
    F: FnOnce() -> String,
{
    function()
}

/// Статический полиморфизм: тип замыкания становится частью типа структуры.
pub struct GenericProcessor<F>
where
    F: Fn(u32) -> u32,
{
    pub function: F,
}

impl<F> GenericProcessor<F>
where
    F: Fn(u32) -> u32,
{
    pub fn process(&self, value: u32) -> u32 {
        (self.function)(value)
    }
}

/// Динамическая диспетчеризация позволяет хранить разные замыкания одного типа.
pub struct DynProcessor {
    pub function: Box<dyn Fn(u32) -> u32>,
}

impl DynProcessor {
    pub fn process(&self, value: u32) -> u32 {
        (self.function)(value)
    }
}

/// HRTB: результат ссылается на первый аргумент при любом времени жизни `'a`.
pub fn use_hrtb<F>(function: F, a: &mut u32) -> &mut u32
where
    F: for<'a> Fn(&'a mut u32, &mut u32) -> &'a mut u32,
{
    let mut local = 42_u32;
    function(a, &mut local)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_pointer_and_closure_share_call_interface() {
        assert_eq!(apply_function(square, 5), 25);

        let multiplier = 3_u32;
        let closure = move |value| u64::from(value * multiplier);
        assert_eq!(closure(5), 15);
    }

    #[test]
    fn fn_closure_can_be_called_repeatedly() {
        let offset = 10;
        let add_offset = |value| value + offset;
        assert_eq!(call_twice(add_offset, 5), (15, 15));
    }

    #[test]
    fn fn_mut_changes_captured_state() {
        let mut sum = 0;
        accumulate_with(|value| sum += value, &[1, 2, 3]);
        assert_eq!(sum, 6);
    }

    #[test]
    fn returned_closure_owns_its_state() {
        let mut adder = get_adder(10);
        assert_eq!(adder(1), 11);
        assert_eq!(adder(2), 13);
        assert_eq!(adder(0), 13);
    }

    #[test]
    fn fn_once_can_move_out_captured_value() {
        let text = String::from("значение перемещено из замыкания");
        let closure = move || text;
        assert_eq!(consume_once(closure), "значение перемещено из замыкания");
    }

    #[test]
    fn closures_can_be_stored() {
        let generic = GenericProcessor {
            function: |value| value * 2,
        };
        let dynamic = DynProcessor {
            function: Box::new(|value| value + 1),
        };

        assert_eq!(generic.process(10), 20);
        assert_eq!(dynamic.process(10), 11);
    }

    #[test]
    fn hrtb_accepts_closure_with_arbitrary_lifetime() {
        let mut value = 10;
        let result = use_hrtb(|first, _second| first, &mut value);
        *result += 5;
        assert_eq!(value, 15);
    }
}
