pub const ESPANK_SUCCESS: u32 = spank_err_ESPANK_SUCCESS;
pub const slurm_err_t_ESPANK_ERROR: u32 = spank_err_ESPANK_ERROR;
pub const slurm_err_t_ESPANK_BAD_ARG: u32 = spank_err_ESPANK_BAD_ARG;
pub const slurm_err_t_ESPANK_NOT_TASK: u32 = spank_err_ESPANK_NOT_TASK;
pub const slurm_err_t_ESPANK_ENV_EXISTS: u32 = spank_err_ESPANK_ENV_EXISTS;
pub const slurm_err_t_ESPANK_ENV_NOEXIST: u32 = spank_err_ESPANK_ENV_NOEXIST;
pub const slurm_err_t_ESPANK_NOSPACE: u32 = spank_err_ESPANK_NOSPACE;
pub const slurm_err_t_ESPANK_NOT_REMOTE: u32 = spank_err_ESPANK_NOT_REMOTE;
pub const slurm_err_t_ESPANK_NOEXIST: u32 = spank_err_ESPANK_NOEXIST;
pub const slurm_err_t_ESPANK_NOT_EXECD: u32 = spank_err_ESPANK_NOT_EXECD;
pub const slurm_err_t_ESPANK_NOT_AVAIL: u32 = spank_err_ESPANK_NOT_AVAIL;
pub const slurm_err_t_ESPANK_NOT_LOCAL: u32 = spank_err_ESPANK_NOT_LOCAL;

pub unsafe fn spank_prepend_task_argv(
    _spank: spank_t,
    _argc: ::std::os::raw::c_int,
    _argv: *mut *const ::std::os::raw::c_char,
) -> spank_err_t {
    slurm_err_t_ESPANK_ERROR
}