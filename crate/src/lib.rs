autocxx::include_cpp! {
	#include "s2e/S2E.h"

	safety!(unsafe_ffi)
	generate!("s2e::S2E")
}
