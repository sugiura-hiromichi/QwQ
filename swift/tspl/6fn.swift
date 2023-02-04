var fst = 0, snd = 3
func omit_fst_arg(_ fst: Int, snd: Int) {
	assert(fst == 0); assert(snd == 3)
}

omit_fst_arg(fst, snd: snd)

func inout_params(_ a: inout Int, _ b: inout Int) {
	(a, b) = (b, a)
	assert(a == 3); assert(b == 0)
}

inout_params(&fst, &snd)

let fn_type = omit_fst_arg
fn_type(snd, fst)
