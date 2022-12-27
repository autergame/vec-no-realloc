macro_rules! SomeBox {
    ($x:expr) => {
        Some(Box::new($x))
    };
}

#[macro_export]
macro_rules! vnr [
    () => {
		$crate::VecNoRealloc::new()
	};
    ($elem:expr; $n:expr) => {
		$crate::VecNoRealloc::from_elem($elem, $n)
	};
	($ ($x:expr) , *) => {
		$crate::VecNoRealloc::<_>::from_slice(&[$ ($x) , *])
    };
];
