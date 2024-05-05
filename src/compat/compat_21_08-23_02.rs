pub unsafe fn spank_prepend_task_argv(
    _spank: spank_t,
    _argc: ::std::os::raw::c_int,
    _argv: *mut *const ::std::os::raw::c_char,
) -> spank_err_t {
    slurm_err_t_ESPANK_ERROR
}