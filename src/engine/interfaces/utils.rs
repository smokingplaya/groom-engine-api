#[macro_export]
macro_rules! define_source_interface {
	($t:path,$n:ident) => {
		paste::paste! {
			thread_local! {
				pub static [<INTERFACE_ $n>]: std::cell::LazyCell<*mut $t> = std::cell::LazyCell::new(|| {
			    use atomicframework_api::ErrorDump;
					use rglua::{iface, cstr};

					let instance = rglua::iface!($t);
					if instance.is_err() {
					  instance.err_dump();
						panic!("invalid interface!")
					}
					instance.unwrap()
				});
			}
		}
	};
	($dn:expr,$in:expr,$t:path,$n:ident) => {
  	paste::paste! {
  		thread_local! {
  			pub static [<INTERFACE_ $n>]: std::cell::LazyCell<*mut $t> = std::cell::LazyCell::new(|| {
  		    use atomicframework_api::ErrorDump;
          use rglua::cstr;

  				let instance = rglua::iface!($dn, $in, $t);
  				if instance.is_err() {
  				  instance.err_dump();
  					panic!("invalid interface!")
  				}
  				instance.unwrap()
  			});
  		}
  	}
	}
}

#[macro_export]
macro_rules! call_interface_function {
  ($i:expr,$m:ident,$($a:expr),*) => {
    $i.with(|interface| {
      let interface = **interface;
      unsafe { // :()
        (*interface).$m($($a),*)
      }
    })
  };
}