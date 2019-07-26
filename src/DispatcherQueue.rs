#![allow(non_camel_case_types, non_snake_case)]
use winapi::shared::minwindef::DWORD;
use winapi::{ENUM, STRUCT};
use winrt::windows::system::IDispatcherQueueController;
use winrt::{ComInterface, HRESULT};

ENUM! {enum DISPATCHERQUEUE_THREAD_APARTMENTTYPE
{
    DQTAT_COM_NONE = 0,
    DQTAT_COM_ASTA = 1,
    DQTAT_COM_STA = 2,
}}

ENUM! {enum DISPATCHERQUEUE_THREAD_TYPE
{
    DQTYPE_THREAD_DEDICATED = 1,
    DQTYPE_THREAD_CURRENT = 2,
}}

STRUCT! {struct DispatcherQueueOptions {
    dwSize: DWORD,
    threadType: DISPATCHERQUEUE_THREAD_TYPE,
    apartmentType: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
}}

extern "system" {
    #[link(name = "CoreMessaging")]
    pub fn CreateDispatcherQueueController(
        options: DispatcherQueueOptions,
        dispatcherQueueController: *mut *mut <IDispatcherQueueController as ComInterface>::TAbi,
    ) -> HRESULT;
}
