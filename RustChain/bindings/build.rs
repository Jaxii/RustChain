fn main() {
    windows::build!(
        Windows::Win32::System::Diagnostics::Debug::{IMAGE_FILE_HEADER,IMAGE_OPTIONAL_HEADER32,IMAGE_SECTION_HEADER,
            IMAGE_DATA_DIRECTORY,IMAGE_OPTIONAL_HEADER_MAGIC,IMAGE_SUBSYSTEM,MINIDUMP_EXCEPTION_INFORMATION,MINIDUMP_USER_STREAM_INFORMATION,
            MINIDUMP_CALLBACK_INFORMATION,GetThreadContext,SetThreadContext},
        Windows::Win32::System::Memory::{VIRTUAL_ALLOCATION_TYPE,PAGE_PROTECTION_FLAGS,MEMORY_BASIC_INFORMATION},
        Windows::Win32::Foundation::{HANDLE,HINSTANCE,PSTR,BOOL},
        Windows::Win32::System::Threading::{GetCurrentProcess,GetCurrentThread,PROCESS_BASIC_INFORMATION,STARTUPINFOW,PROCESS_INFORMATION},
        Windows::Win32::System::SystemServices::{IMAGE_BASE_RELOCATION,IMAGE_IMPORT_DESCRIPTOR,IMAGE_THUNK_DATA32,IMAGE_THUNK_DATA64},
        Windows::Win32::System::WindowsProgramming::{PUBLIC_OBJECT_TYPE_INFORMATION,OBJECT_ATTRIBUTES,CLIENT_ID},
        Windows::Win32::Security::SECURITY_ATTRIBUTES,
        Windows::Win32::System::WindowsProgramming::IO_STATUS_BLOCK,
        Windows::Win32::System::SystemInformation::SYSTEM_INFO
    );
}