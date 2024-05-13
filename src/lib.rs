pub mod events;
pub mod messages;
pub mod requests;
mod types;

pub use crate::types::*;

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            supports_configuration_done_request: None,
            supports_function_breakpoints: None,
            supports_conditional_breakpoints: None,
            supports_hit_conditional_breakpoints: None,
            supports_evaluate_for_hovers: None,
            exception_breakpoint_filters: None,
            supports_step_back: None,
            supports_set_variable: None,
            supports_restart_frame: None,
            supports_goto_targets_request: None,
            supports_step_in_targets_request: None,
            supports_completions_request: None,
            completion_trigger_characters: None,
            supports_modules_request: None,
            additional_module_columns: None,
            supported_checksum_algorithms: None,
            supports_restart_request: None,
            supports_exception_options: None,
            supports_value_formatting_options: None,
            supports_exception_info_request: None,
            support_terminate_debuggee: None,
            support_suspend_debuggee: None,
            supports_delayed_stack_trace_loading: None,
            supports_loaded_sources_request: None,
            supports_log_points: None,
            supports_terminate_threads_request: None,
            supports_set_expression: None,
            supports_terminate_request: None,
            supports_data_breakpoints: None,
            supports_read_memory_request: None,
            supports_write_memory_request: None,
            supports_disassemble_request: None,
            supports_cancel_request: None,
            supports_breakpoint_locations_request: None,
            supports_clipboard_context: None,
            supports_stepping_granularity: None,
            supports_instruction_breakpoints: None,
            supports_exception_filter_options: None,
            supports_single_thread_execution_requests: None,
            breakpoint_modes: None,
        }
    }
}
