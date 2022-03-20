#[allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Windows {
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Foundation {
        #[repr(C)]
        #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
        pub struct TimeSpan {
            pub Duration: i64,
        }
        impl TimeSpan {}
        impl ::std::default::Default for TimeSpan {
            fn default() -> Self {
                Self { Duration: 0 }
            }
        }
        impl ::std::fmt::Debug for TimeSpan {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                fmt.debug_struct("TimeSpan")
                    .field("Duration", &format_args!("{:?}", self.Duration))
                    .finish()
            }
        }
        impl ::std::cmp::PartialEq for TimeSpan {
            fn eq(&self, other: &Self) -> bool {
                self.Duration == other.Duration
            }
        }
        impl ::std::cmp::Eq for TimeSpan {}
        unsafe impl ::windows::Abi for TimeSpan {
            type Abi = Self;
        }
        unsafe impl ::windows::RuntimeType for TimeSpan {
            type DefaultType = Self;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"struct(Windows.Foundation.TimeSpan;i8)");
        }
        impl ::std::convert::From<::std::time::Duration> for TimeSpan {
            fn from(value: ::std::time::Duration) -> Self {
                Self {
                    Duration: (value.as_nanos() / 100) as i64,
                }
            }
        }
        impl ::std::convert::From<TimeSpan> for ::std::time::Duration {
            fn from(value: TimeSpan) -> Self {
                ::std::time::Duration::from_nanos((value.Duration * 100) as u64)
            }
        }
        impl<'a> ::windows::IntoParam<'a, TimeSpan> for ::std::time::Duration {
            fn into_param(self) -> ::windows::Param<'a, TimeSpan> {
                ::windows::Param::Owned(self.into())
            }
        }
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IAsyncInfo(::windows::Object);
        unsafe impl ::windows::Interface for IAsyncInfo {
            type Vtable = IAsyncInfo_abi;
            const IID: ::windows::Guid =
                ::windows::Guid::from_values(54, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
        }
        impl IAsyncInfo {
            pub fn Id(&self) -> ::windows::Result<u32> {
                let this = self;
                unsafe {
                    let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<u32>(result__)
                }
            }
            pub fn Status(&self) -> ::windows::Result<AsyncStatus> {
                let this = self;
                unsafe {
                    let mut result__: <AsyncStatus as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<AsyncStatus>(result__)
                }
            }
            pub fn ErrorCode(&self) -> ::windows::Result<::windows::HRESULT> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HRESULT as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).8)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HRESULT>(result__)
                }
            }
            pub fn Cancel(&self) -> ::windows::Result<()> {
                let this = self;
                unsafe { (::windows::Interface::vtable(this).9)(::windows::Abi::abi(this)).ok() }
            }
            pub fn Close(&self) -> ::windows::Result<()> {
                let this = self;
                unsafe { (::windows::Interface::vtable(this).10)(::windows::Abi::abi(this)).ok() }
            }
        }
        unsafe impl ::windows::RuntimeType for IAsyncInfo {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{00000036-0000-0000-c000-000000000046}");
        }
        impl ::std::convert::From<IAsyncInfo> for ::windows::Object {
            fn from(value: IAsyncInfo) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IAsyncInfo> for ::windows::Object {
            fn from(value: &IAsyncInfo) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IAsyncInfo {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IAsyncInfo {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IAsyncInfo_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut u32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut AsyncStatus,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::HRESULT,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
        );
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: marker :: Copy,
            :: std :: clone :: Clone,
            :: std :: default :: Default,
            :: std :: fmt :: Debug,
        )]
        #[repr(transparent)]
        pub struct AsyncStatus(pub i32);
        impl AsyncStatus {
            pub const Canceled: Self = Self(2i32);
            pub const Completed: Self = Self(1i32);
            pub const Error: Self = Self(3i32);
            pub const Started: Self = Self(0i32);
        }
        impl ::std::convert::From<i32> for AsyncStatus {
            fn from(value: i32) -> Self {
                Self(value)
            }
        }
        unsafe impl ::windows::Abi for AsyncStatus {
            type Abi = Self;
        }
        unsafe impl ::windows::RuntimeType for AsyncStatus {
            type DefaultType = Self;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"enum(Windows.Foundation.AsyncStatus;i4)");
        }
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct AsyncOperationCompletedHandler<TResult>(
            ::windows::IUnknown,
            ::std::marker::PhantomData<TResult>,
        )
        where
            TResult: ::windows::RuntimeType + 'static;
        impl<TResult: ::windows::RuntimeType + 'static> AsyncOperationCompletedHandler<TResult> {
            pub fn new<
                F: FnMut(
                        &::std::option::Option<IAsyncOperation<TResult>>,
                        AsyncStatus,
                    ) -> ::windows::Result<()>
                    + 'static,
            >(
                invoke: F,
            ) -> Self {
                let com = AsyncOperationCompletedHandler_box::<TResult, F> {
                    vtable: &AsyncOperationCompletedHandler_box::<TResult, F>::VTABLE,
                    count: ::windows::RefCount::new(),
                    invoke,
                };
                unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
            }
            pub fn Invoke<'a>(
                &self,
                asyncinfo: impl ::windows::IntoParam<'a, IAsyncOperation<TResult>>,
                asyncstatus: AsyncStatus,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).3)(
                        ::windows::Abi::abi(this),
                        asyncinfo.into_param().abi(),
                        asyncstatus,
                    )
                    .ok()
                }
            }
        }
        unsafe impl<TResult: ::windows::RuntimeType + 'static> ::windows::RuntimeType
            for AsyncOperationCompletedHandler<TResult>
        {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer = {
                ::windows::ConstBuffer::new()
                    .push_slice(b"pinterface(")
                    .push_slice(b"{fcdcf02c-e5d8-4478-915a-4d90b74b83a5}")
                    .push_slice(b";")
                    .push_other(<TResult as ::windows::RuntimeType>::SIGNATURE)
                    .push_slice(b")")
            };
        }
        unsafe impl<TResult: ::windows::RuntimeType + 'static> ::windows::Interface
            for AsyncOperationCompletedHandler<TResult>
        {
            type Vtable = AsyncOperationCompletedHandler_abi<TResult>;
            const IID: ::windows::Guid = ::windows::Guid::from_signature(
                <AsyncOperationCompletedHandler<TResult> as ::windows::RuntimeType>::SIGNATURE,
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct AsyncOperationCompletedHandler_abi<TResult>(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                asyncinfo: ::windows::RawPtr,
                asyncstatus: AsyncStatus,
            ) -> ::windows::HRESULT,
            pub ::std::marker::PhantomData<TResult>,
        )
        where
            TResult: ::windows::RuntimeType + 'static;
        #[repr(C)]
        struct AsyncOperationCompletedHandler_box<
            TResult,
            F: FnMut(
                    &::std::option::Option<IAsyncOperation<TResult>>,
                    AsyncStatus,
                ) -> ::windows::Result<()>
                + 'static,
        >
        where
            TResult: ::windows::RuntimeType + 'static,
        {
            vtable: *const AsyncOperationCompletedHandler_abi<TResult>,
            invoke: F,
            count: ::windows::RefCount,
        }
        impl<
                TResult: ::windows::RuntimeType + 'static,
                F: FnMut(
                        &::std::option::Option<IAsyncOperation<TResult>>,
                        AsyncStatus,
                    ) -> ::windows::Result<()>
                    + 'static,
            > AsyncOperationCompletedHandler_box<TResult, F>
        {
            const VTABLE: AsyncOperationCompletedHandler_abi<TResult> =
                AsyncOperationCompletedHandler_abi::<TResult>(
                    Self::QueryInterface,
                    Self::AddRef,
                    Self::Release,
                    Self::Invoke,
                    ::std::marker::PhantomData::<TResult>,
                );
            unsafe extern "system" fn QueryInterface(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                *interface = if iid
                    == &<AsyncOperationCompletedHandler<TResult> as ::windows::Interface>::IID
                    || iid == &<::windows::IUnknown as ::windows::Interface>::IID
                    || iid == &<::windows::IAgileObject as ::windows::Interface>::IID
                {
                    &mut (*this).vtable as *mut _ as _
                } else {
                    ::std::ptr::null_mut()
                };
                if (*interface).is_null() {
                    ::windows::HRESULT(0x8000_4002)
                } else {
                    (*this).count.add_ref();
                    ::windows::HRESULT(0)
                }
            }
            unsafe extern "system" fn AddRef(this: ::windows::RawPtr) -> u32 {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                (*this).count.add_ref()
            }
            unsafe extern "system" fn Release(this: ::windows::RawPtr) -> u32 {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                let remaining = (*this).count.release();
                if remaining == 0 {
                    Box::from_raw(this);
                }
                remaining
            }
            unsafe extern "system" fn Invoke(
                this: ::windows::RawPtr,
                asyncinfo: ::windows::RawPtr,
                asyncstatus: AsyncStatus,
            ) -> ::windows::HRESULT {
                let this = this as *mut ::windows::RawPtr as *mut Self;
                ( ( * this ) . invoke ) ( & * ( & asyncinfo as * const < IAsyncOperation < TResult > as :: windows :: Abi > :: Abi as * const < IAsyncOperation < TResult > as :: windows :: RuntimeType > :: DefaultType ) , asyncstatus , ) . into ( )
            }
        }
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IAsyncOperation<TResult>(::windows::Object, ::std::marker::PhantomData<TResult>)
        where
            TResult: ::windows::RuntimeType + 'static;
        unsafe impl<TResult: ::windows::RuntimeType + 'static> ::windows::Interface
            for IAsyncOperation<TResult>
        {
            type Vtable = IAsyncOperation_abi<TResult>;
            const IID: ::windows::Guid = ::windows::Guid::from_signature(
                <IAsyncOperation<TResult> as ::windows::RuntimeType>::SIGNATURE,
            );
        }
        impl<TResult: ::windows::RuntimeType + 'static> IAsyncOperation<TResult> {
            pub fn SetCompleted<'a>(
                &self,
                handler: impl ::windows::IntoParam<'a, AsyncOperationCompletedHandler<TResult>>,
            ) -> ::windows::Result<()> {
                let this = self;
                unsafe {
                    (::windows::Interface::vtable(this).6)(
                        ::windows::Abi::abi(this),
                        handler.into_param().abi(),
                    )
                    .ok()
                }
            }
            pub fn Completed(&self) -> ::windows::Result<AsyncOperationCompletedHandler<TResult>> {
                let this = self;
                unsafe {
                    let mut result__ : < AsyncOperationCompletedHandler < TResult > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<AsyncOperationCompletedHandler<TResult>>(result__)
                }
            }
            pub fn GetResults(&self) -> ::windows::Result<TResult> {
                let this = self;
                unsafe {
                    let mut result__: <TResult as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).8)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<TResult>(result__)
                }
            }
            pub fn Id(&self) -> ::windows::Result<u32> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self).unwrap();
                unsafe {
                    let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<u32>(result__)
                }
            }
            pub fn Status(&self) -> ::windows::Result<AsyncStatus> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self).unwrap();
                unsafe {
                    let mut result__: <AsyncStatus as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<AsyncStatus>(result__)
                }
            }
            pub fn ErrorCode(&self) -> ::windows::Result<::windows::HRESULT> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HRESULT as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).8)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HRESULT>(result__)
                }
            }
            pub fn Cancel(&self) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self).unwrap();
                unsafe { (::windows::Interface::vtable(this).9)(::windows::Abi::abi(this)).ok() }
            }
            pub fn Close(&self) -> ::windows::Result<()> {
                let this = &::windows::Interface::cast::<IAsyncInfo>(self).unwrap();
                unsafe { (::windows::Interface::vtable(this).10)(::windows::Abi::abi(this)).ok() }
            }
            pub fn get(&self) -> ::windows::Result<TResult> {
                if self.Status()? == AsyncStatus::Started {
                    let (waiter, signaler) = ::windows::Waiter::new();
                    self.SetCompleted(AsyncOperationCompletedHandler::new(
                        move |_sender, _args| {
                            unsafe {
                                signaler.signal();
                            }
                            Ok(())
                        },
                    ))?;
                }
                self.GetResults()
            }
        }
        unsafe impl<TResult: ::windows::RuntimeType + 'static> ::windows::RuntimeType
            for IAsyncOperation<TResult>
        {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer = {
                ::windows::ConstBuffer::new()
                    .push_slice(b"pinterface(")
                    .push_slice(b"{9fc2b0bb-e446-44e2-aa61-9cab8f636af2}")
                    .push_slice(b";")
                    .push_other(<TResult as ::windows::RuntimeType>::SIGNATURE)
                    .push_slice(b")")
            };
        }
        impl<TResult: ::windows::RuntimeType + 'static> ::std::future::Future for IAsyncOperation<TResult> {
            type Output = ::windows::Result<TResult>;
            fn poll(
                self: ::std::pin::Pin<&mut Self>,
                context: &mut ::std::task::Context,
            ) -> ::std::task::Poll<Self::Output> {
                if self.Status()? == AsyncStatus::Started {
                    let waker = context.waker().clone();
                    let _ = self.SetCompleted(AsyncOperationCompletedHandler::new(
                        move |_sender, _args| {
                            waker.wake_by_ref();
                            Ok(())
                        },
                    ));
                    ::std::task::Poll::Pending
                } else {
                    ::std::task::Poll::Ready(self.GetResults())
                }
            }
        }
        impl<TResult: ::windows::RuntimeType + 'static>
            ::std::convert::From<IAsyncOperation<TResult>> for ::windows::Object
        {
            fn from(value: IAsyncOperation<TResult>) -> Self {
                value.0
            }
        }
        impl<TResult: ::windows::RuntimeType + 'static>
            ::std::convert::From<&IAsyncOperation<TResult>> for ::windows::Object
        {
            fn from(value: &IAsyncOperation<TResult>) -> Self {
                value.0.clone()
            }
        }
        impl<'a, TResult: ::windows::RuntimeType + 'static>
            ::windows::IntoParam<'a, ::windows::Object> for IAsyncOperation<TResult>
        {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a, TResult: ::windows::RuntimeType + 'static>
            ::windows::IntoParam<'a, ::windows::Object> for &'a IAsyncOperation<TResult>
        {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        impl<TResult: ::windows::RuntimeType + 'static>
            ::std::convert::From<IAsyncOperation<TResult>> for IAsyncInfo
        {
            fn from(value: IAsyncOperation<TResult>) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl<TResult: ::windows::RuntimeType + 'static>
            ::std::convert::From<&IAsyncOperation<TResult>> for IAsyncInfo
        {
            fn from(value: &IAsyncOperation<TResult>) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a, TResult: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IAsyncInfo>
            for IAsyncOperation<TResult>
        {
            fn into_param(self) -> ::windows::Param<'a, IAsyncInfo> {
                ::windows::Param::Owned(::std::convert::Into::<IAsyncInfo>::into(self))
            }
        }
        impl<'a, TResult: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IAsyncInfo>
            for &'a IAsyncOperation<TResult>
        {
            fn into_param(self) -> ::windows::Param<'a, IAsyncInfo> {
                ::windows::Param::Owned(::std::convert::Into::<IAsyncInfo>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        unsafe impl<TResult: ::windows::RuntimeType + 'static> ::std::marker::Send
            for IAsyncOperation<TResult>
        {
        }
        unsafe impl<TResult: ::windows::RuntimeType + 'static> ::std::marker::Sync
            for IAsyncOperation<TResult>
        {
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IAsyncOperation_abi<TResult>(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                handler: ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut <TResult as ::windows::Abi>::Abi,
            ) -> ::windows::HRESULT,
            pub ::std::marker::PhantomData<TResult>,
        )
        where
            TResult: ::windows::RuntimeType + 'static;
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IClosable(::windows::Object);
        unsafe impl ::windows::Interface for IClosable {
            type Vtable = IClosable_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                819308585,
                32676,
                16422,
                [131, 187, 215, 91, 174, 78, 169, 158],
            );
        }
        impl IClosable {
            pub fn Close(&self) -> ::windows::Result<()> {
                let this = self;
                unsafe { (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this)).ok() }
            }
        }
        unsafe impl ::windows::RuntimeType for IClosable {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{30d5a829-7fa4-4026-83bb-d75bae4ea99e}");
        }
        impl ::std::convert::From<IClosable> for ::windows::Object {
            fn from(value: IClosable) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IClosable> for ::windows::Object {
            fn from(value: &IClosable) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IClosable {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IClosable {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IClosable_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
        );
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Collections {
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IIterator<T>(::windows::Object, ::std::marker::PhantomData<T>)
            where
                T: ::windows::RuntimeType + 'static;
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::Interface for IIterator<T> {
                type Vtable = IIterator_abi<T>;
                const IID: ::windows::Guid = ::windows::Guid::from_signature(
                    <IIterator<T> as ::windows::RuntimeType>::SIGNATURE,
                );
            }
            impl<T: ::windows::RuntimeType + 'static> IIterator<T> {
                pub fn Current(&self) -> ::windows::Result<T> {
                    let this = self;
                    unsafe {
                        let mut result__: <T as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<T>(result__)
                    }
                }
                pub fn HasCurrent(&self) -> ::windows::Result<bool> {
                    let this = self;
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn MoveNext(&self) -> ::windows::Result<bool> {
                    let this = self;
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn GetMany(
                    &self,
                    items: &mut [<T as ::windows::RuntimeType>::DefaultType],
                ) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            items.len() as u32,
                            ::std::mem::transmute_copy(&items),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
            }
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::RuntimeType for IIterator<T> {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer = {
                    ::windows::ConstBuffer::new()
                        .push_slice(b"pinterface(")
                        .push_slice(b"{6a79e863-4300-459a-9966-cbb660963ee1}")
                        .push_slice(b";")
                        .push_other(<T as ::windows::RuntimeType>::SIGNATURE)
                        .push_slice(b")")
                };
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IIterator<T>> for ::windows::Object {
                fn from(value: IIterator<T>) -> Self {
                    value.0
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IIterator<T>>
                for ::windows::Object
            {
                fn from(value: &IIterator<T>) -> Self {
                    value.0.clone()
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::Object> for IIterator<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::Object> for &'a IIterator<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::Iterator for IIterator<T> {
                type Item = T;
                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    let result = self.Current().ok();
                    if result.is_some() {
                        self.MoveNext().ok()?;
                    }
                    result
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IIterator_abi<T>(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut bool,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut bool,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    items_array_size: u32,
                    items: *mut <T as ::windows::Abi>::Abi,
                    result__: *mut u32,
                ) -> ::windows::HRESULT,
                pub ::std::marker::PhantomData<T>,
            )
            where
                T: ::windows::RuntimeType + 'static;
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IIterable<T>(::windows::Object, ::std::marker::PhantomData<T>)
            where
                T: ::windows::RuntimeType + 'static;
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::Interface for IIterable<T> {
                type Vtable = IIterable_abi<T>;
                const IID: ::windows::Guid = ::windows::Guid::from_signature(
                    <IIterable<T> as ::windows::RuntimeType>::SIGNATURE,
                );
            }
            impl<T: ::windows::RuntimeType + 'static> IIterable<T> {
                pub fn First(&self) -> ::windows::Result<IIterator<T>> {
                    let this = self;
                    unsafe {
                        let mut result__: <IIterator<T> as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<IIterator<T>>(result__)
                    }
                }
            }
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::RuntimeType for IIterable<T> {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer = {
                    ::windows::ConstBuffer::new()
                        .push_slice(b"pinterface(")
                        .push_slice(b"{faa585ea-6214-4217-afda-7f46de5869b3}")
                        .push_slice(b";")
                        .push_other(<T as ::windows::RuntimeType>::SIGNATURE)
                        .push_slice(b")")
                };
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IIterable<T>> for ::windows::Object {
                fn from(value: IIterable<T>) -> Self {
                    value.0
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IIterable<T>>
                for ::windows::Object
            {
                fn from(value: &IIterable<T>) -> Self {
                    value.0.clone()
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::Object> for IIterable<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::Object> for &'a IIterable<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::IntoIterator for IIterable<T> {
                type Item = T;
                type IntoIter = IIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    self.First().unwrap()
                }
            }
            impl<'a, T: ::windows::RuntimeType> ::std::iter::IntoIterator for &'a IIterable<T> {
                type Item = T;
                type IntoIter = IIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    self.First().unwrap()
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IIterable_abi<T>(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub ::std::marker::PhantomData<T>,
            )
            where
                T: ::windows::RuntimeType + 'static;
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IVectorView<T>(::windows::Object, ::std::marker::PhantomData<T>)
            where
                T: ::windows::RuntimeType + 'static;
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::Interface for IVectorView<T> {
                type Vtable = IVectorView_abi<T>;
                const IID: ::windows::Guid = ::windows::Guid::from_signature(
                    <IVectorView<T> as ::windows::RuntimeType>::SIGNATURE,
                );
            }
            impl<T: ::windows::RuntimeType + 'static> IVectorView<T> {
                pub fn GetAt(&self, index: u32) -> ::windows::Result<T> {
                    let this = self;
                    unsafe {
                        let mut result__: <T as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            index,
                            &mut result__,
                        )
                        .from_abi::<T>(result__)
                    }
                }
                pub fn Size(&self) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
                pub fn IndexOf<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, T>,
                    index: &mut u32,
                ) -> ::windows::Result<bool> {
                    let this = self;
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                            index,
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn GetMany(
                    &self,
                    startindex: u32,
                    items: &mut [<T as ::windows::RuntimeType>::DefaultType],
                ) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            startindex,
                            items.len() as u32,
                            ::std::mem::transmute_copy(&items),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
                pub fn First(&self) -> ::windows::Result<IIterator<T>> {
                    let this = &::windows::Interface::cast::<IIterable<T>>(self).unwrap();
                    unsafe {
                        let mut result__: <IIterator<T> as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<IIterator<T>>(result__)
                    }
                }
            }
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::RuntimeType for IVectorView<T> {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer = {
                    ::windows::ConstBuffer::new()
                        .push_slice(b"pinterface(")
                        .push_slice(b"{bbe1fa4c-b0e3-4583-baef-1f1b2e483e56}")
                        .push_slice(b";")
                        .push_other(<T as ::windows::RuntimeType>::SIGNATURE)
                        .push_slice(b")")
                };
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IVectorView<T>>
                for ::windows::Object
            {
                fn from(value: IVectorView<T>) -> Self {
                    value.0
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IVectorView<T>>
                for ::windows::Object
            {
                fn from(value: &IVectorView<T>) -> Self {
                    value.0.clone()
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::Object> for IVectorView<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::Object> for &'a IVectorView<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IVectorView<T>> for IIterable<T> {
                fn from(value: IVectorView<T>) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IVectorView<T>> for IIterable<T> {
                fn from(value: &IVectorView<T>) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IIterable<T>>
                for IVectorView<T>
            {
                fn into_param(self) -> ::windows::Param<'a, IIterable<T>> {
                    ::windows::Param::Owned(::std::convert::Into::<IIterable<T>>::into(self))
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IIterable<T>>
                for &'a IVectorView<T>
            {
                fn into_param(self) -> ::windows::Param<'a, IIterable<T>> {
                    ::windows::Param::Owned(::std::convert::Into::<IIterable<T>>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            pub struct VectorViewIterator<T: ::windows::RuntimeType + 'static> {
                vector: IVectorView<T>,
                current: u32,
                size: u32,
            }
            impl<T: ::windows::RuntimeType> VectorViewIterator<T> {
                pub fn new(vector: IVectorView<T>) -> Self {
                    let size = vector.Size().unwrap();
                    Self {
                        vector,
                        current: 0,
                        size,
                    }
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::Iterator for VectorViewIterator<T> {
                type Item = T;
                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    if self.current >= self.size {
                        return None;
                    }
                    let result = self.vector.GetAt(self.current);
                    self.current += 1;
                    result.ok()
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::IntoIterator for IVectorView<T> {
                type Item = T;
                type IntoIter = VectorViewIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    VectorViewIterator::new(self)
                }
            }
            impl<'a, T: ::windows::RuntimeType> ::std::iter::IntoIterator for &'a IVectorView<T> {
                type Item = T;
                type IntoIter = VectorViewIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    VectorViewIterator::new(::std::clone::Clone::clone(self))
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IVectorView_abi<T>(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    index: u32,
                    result__: *mut <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut u32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: <T as ::windows::Abi>::Abi,
                    index: *mut u32,
                    result__: *mut bool,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    startindex: u32,
                    items_array_size: u32,
                    items: *mut <T as ::windows::Abi>::Abi,
                    result__: *mut u32,
                ) -> ::windows::HRESULT,
                pub ::std::marker::PhantomData<T>,
            )
            where
                T: ::windows::RuntimeType + 'static;
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IVector<T>(::windows::Object, ::std::marker::PhantomData<T>)
            where
                T: ::windows::RuntimeType + 'static;
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::Interface for IVector<T> {
                type Vtable = IVector_abi<T>;
                const IID: ::windows::Guid = ::windows::Guid::from_signature(
                    <IVector<T> as ::windows::RuntimeType>::SIGNATURE,
                );
            }
            impl<T: ::windows::RuntimeType + 'static> IVector<T> {
                pub fn GetAt(&self, index: u32) -> ::windows::Result<T> {
                    let this = self;
                    unsafe {
                        let mut result__: <T as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            index,
                            &mut result__,
                        )
                        .from_abi::<T>(result__)
                    }
                }
                pub fn Size(&self) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
                pub fn GetView(&self) -> ::windows::Result<IVectorView<T>> {
                    let this = self;
                    unsafe {
                        let mut result__: <IVectorView<T> as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<IVectorView<T>>(result__)
                    }
                }
                pub fn IndexOf<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, T>,
                    index: &mut u32,
                ) -> ::windows::Result<bool> {
                    let this = self;
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                            index,
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn SetAt<'a>(
                    &self,
                    index: u32,
                    value: impl ::windows::IntoParam<'a, T>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).10)(
                            ::windows::Abi::abi(this),
                            index,
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn InsertAt<'a>(
                    &self,
                    index: u32,
                    value: impl ::windows::IntoParam<'a, T>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).11)(
                            ::windows::Abi::abi(this),
                            index,
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn RemoveAt(&self, index: u32) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).12)(::windows::Abi::abi(this), index)
                            .ok()
                    }
                }
                pub fn Append<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, T>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).13)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn RemoveAtEnd(&self) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).14)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Clear(&self) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).15)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn GetMany(
                    &self,
                    startindex: u32,
                    items: &mut [<T as ::windows::RuntimeType>::DefaultType],
                ) -> ::windows::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__: <u32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).16)(
                            ::windows::Abi::abi(this),
                            startindex,
                            items.len() as u32,
                            ::std::mem::transmute_copy(&items),
                            &mut result__,
                        )
                        .from_abi::<u32>(result__)
                    }
                }
                pub fn ReplaceAll(
                    &self,
                    items: &[<T as ::windows::RuntimeType>::DefaultType],
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).17)(
                            ::windows::Abi::abi(this),
                            items.len() as u32,
                            ::std::mem::transmute(items.as_ptr()),
                        )
                        .ok()
                    }
                }
                pub fn First(&self) -> ::windows::Result<IIterator<T>> {
                    let this = &::windows::Interface::cast::<IIterable<T>>(self).unwrap();
                    unsafe {
                        let mut result__: <IIterator<T> as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<IIterator<T>>(result__)
                    }
                }
            }
            unsafe impl<T: ::windows::RuntimeType + 'static> ::windows::RuntimeType for IVector<T> {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer = {
                    ::windows::ConstBuffer::new()
                        .push_slice(b"pinterface(")
                        .push_slice(b"{913337e9-11a1-4345-a3a2-4e7f956e222d}")
                        .push_slice(b";")
                        .push_other(<T as ::windows::RuntimeType>::SIGNATURE)
                        .push_slice(b")")
                };
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IVector<T>> for ::windows::Object {
                fn from(value: IVector<T>) -> Self {
                    value.0
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IVector<T>> for ::windows::Object {
                fn from(value: &IVector<T>) -> Self {
                    value.0.clone()
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::Object> for IVector<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static>
                ::windows::IntoParam<'a, ::windows::Object> for &'a IVector<T>
            {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<IVector<T>> for IIterable<T> {
                fn from(value: IVector<T>) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl<T: ::windows::RuntimeType + 'static> ::std::convert::From<&IVector<T>> for IIterable<T> {
                fn from(value: &IVector<T>) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IIterable<T>>
                for IVector<T>
            {
                fn into_param(self) -> ::windows::Param<'a, IIterable<T>> {
                    ::windows::Param::Owned(::std::convert::Into::<IIterable<T>>::into(self))
                }
            }
            impl<'a, T: ::windows::RuntimeType + 'static> ::windows::IntoParam<'a, IIterable<T>>
                for &'a IVector<T>
            {
                fn into_param(self) -> ::windows::Param<'a, IIterable<T>> {
                    ::windows::Param::Owned(::std::convert::Into::<IIterable<T>>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            pub struct VectorIterator<T: ::windows::RuntimeType + 'static> {
                vector: IVector<T>,
                current: u32,
                size: u32,
            }
            impl<T: ::windows::RuntimeType> VectorIterator<T> {
                pub fn new(vector: IVector<T>) -> Self {
                    let size = vector.Size().unwrap();
                    Self {
                        vector,
                        current: 0,
                        size,
                    }
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::Iterator for VectorIterator<T> {
                type Item = T;
                fn next(&mut self) -> ::std::option::Option<Self::Item> {
                    if self.current >= self.size {
                        return None;
                    }
                    let result = self.vector.GetAt(self.current);
                    self.current += 1;
                    result.ok()
                }
            }
            impl<T: ::windows::RuntimeType> ::std::iter::IntoIterator for IVector<T> {
                type Item = T;
                type IntoIter = VectorIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    VectorIterator::new(self)
                }
            }
            impl<'a, T: ::windows::RuntimeType> ::std::iter::IntoIterator for &'a IVector<T> {
                type Item = T;
                type IntoIter = VectorIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    VectorIterator::new(::std::clone::Clone::clone(self))
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IVector_abi<T>(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    index: u32,
                    result__: *mut <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut u32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: <T as ::windows::Abi>::Abi,
                    index: *mut u32,
                    result__: *mut bool,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    index: u32,
                    value: <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    index: u32,
                    value: <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    index: u32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    startindex: u32,
                    items_array_size: u32,
                    items: *mut <T as ::windows::Abi>::Abi,
                    result__: *mut u32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    items_array_size: u32,
                    items: *const <T as ::windows::Abi>::Abi,
                ) -> ::windows::HRESULT,
                pub ::std::marker::PhantomData<T>,
            )
            where
                T: ::windows::RuntimeType + 'static;
        }
    }
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Media {
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Audio {
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IAudioGraph(::windows::Object);
            unsafe impl ::windows::Interface for IAudioGraph {
                type Vtable = IAudioGraph_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    450129645,
                    58508,
                    19988,
                    [150, 96, 44, 79, 131, 233, 205, 216],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAudioGraph_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    file: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut u64,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut i32,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IAudioGraph2(::windows::Object);
            unsafe impl ::windows::Interface for IAudioGraph2 {
                type Vtable = IAudioGraph2_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    1313618901,
                    20417,
                    17910,
                    [169, 71, 60, 211, 143, 79, 216, 57],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAudioGraph2_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IAudioGraph3(::windows::Object);
            unsafe impl ::windows::Interface for IAudioGraph3 {
                type Vtable = IAudioGraph3_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    3721209262,
                    4485,
                    17063,
                    [131, 29, 106, 155, 15, 200, 104, 32],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAudioGraph3_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IAudioGraphStatics(::windows::Object);
            unsafe impl ::windows::Interface for IAudioGraphStatics {
                type Vtable = IAudioGraphStatics_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    1995190578,
                    57689,
                    19127,
                    [168, 42, 23, 190, 180, 179, 30, 148],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAudioGraphStatics_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    settings: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct AudioGraph(::windows::Object);
            impl AudioGraph {
                pub fn CreateDeviceOutputNodeAsync(
                    &self,
                ) -> ::windows::Result<
                    super::super::Foundation::IAsyncOperation<CreateAudioDeviceOutputNodeResult>,
                > {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::IAsyncOperation<
                            CreateAudioDeviceOutputNodeResult,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        ( :: windows :: Interface :: vtable ( this ) .13 ) ( :: windows :: Abi :: abi ( this ) , & mut result__ ) . from_abi :: < super :: super :: Foundation :: IAsyncOperation :: < CreateAudioDeviceOutputNodeResult > > ( result__ )
                    }
                }
                pub fn CreateFileInputNodeAsync<'a>(
                    &self,
                    file: impl ::windows::IntoParam<'a, super::super::Storage::IStorageFile>,
                ) -> ::windows::Result<
                    super::super::Foundation::IAsyncOperation<CreateAudioFileInputNodeResult>,
                > {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::IAsyncOperation<
                            CreateAudioFileInputNodeResult,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        ( :: windows :: Interface :: vtable ( this ) .14 ) ( :: windows :: Abi :: abi ( this ) , file . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super :: super :: Foundation :: IAsyncOperation :: < CreateAudioFileInputNodeResult > > ( result__ )
                    }
                }
                pub fn Start(&self) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).19)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Stop(&self) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).20)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn ResetAllNodes(&self) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).21)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn CompletedQuantumCount(&self) -> ::windows::Result<u64> {
                    let this = self;
                    unsafe {
                        let mut result__: <u64 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).28)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<u64>(result__)
                    }
                }
                pub fn LatencyInSamples(&self) -> ::windows::Result<i32> {
                    let this = self;
                    unsafe {
                        let mut result__: <i32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).30)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<i32>(result__)
                    }
                }
                pub fn SamplesPerQuantum(&self) -> ::windows::Result<i32> {
                    let this = self;
                    unsafe {
                        let mut result__: <i32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).33)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<i32>(result__)
                    }
                }
                pub fn Close(&self) -> ::windows::Result<()> {
                    let this =
                        &::windows::Interface::cast::<super::super::Foundation::IClosable>(self)
                            .unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn CreateAsync<'a>(
                    settings: impl ::windows::IntoParam<'a, AudioGraphSettings>,
                ) -> ::windows::Result<
                    super::super::Foundation::IAsyncOperation<CreateAudioGraphResult>,
                > {
                    Self::IAudioGraphStatics(|this| unsafe {
                        let mut result__: <super::super::Foundation::IAsyncOperation<
                            CreateAudioGraphResult,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        ( :: windows :: Interface :: vtable ( this ) .6 ) ( :: windows :: Abi :: abi ( this ) , settings . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super :: super :: Foundation :: IAsyncOperation :: < CreateAudioGraphResult > > ( result__ )
                    })
                }
                fn IAudioGraphStatics<R, F: FnOnce(&IAudioGraphStatics) -> ::windows::Result<R>>(
                    callback: F,
                ) -> ::windows::Result<R> {
                    static mut SHARED: ::windows::FactoryCache<AudioGraph, IAudioGraphStatics> =
                        ::windows::FactoryCache::new();
                    unsafe { SHARED.call(callback) }
                }
            }
            unsafe impl ::windows::RuntimeType for AudioGraph {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                    b"rc(Windows.Media.Audio.AudioGraph;{1ad46eed-e48c-4e14-9660-2c4f83e9cdd8})",
                );
            }
            unsafe impl ::windows::Interface for AudioGraph {
                type Vtable = IAudioGraph_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    450129645,
                    58508,
                    19988,
                    [150, 96, 44, 79, 131, 233, 205, 216],
                );
            }
            impl ::windows::RuntimeName for AudioGraph {
                const NAME: &'static str = "Windows.Media.Audio.AudioGraph";
            }
            impl ::std::convert::From<AudioGraph> for ::windows::Object {
                fn from(value: AudioGraph) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&AudioGraph> for ::windows::Object {
                fn from(value: &AudioGraph) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for AudioGraph {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a AudioGraph {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl ::std::convert::From<AudioGraph> for super::super::Foundation::IClosable {
                fn from(value: AudioGraph) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&AudioGraph> for super::super::Foundation::IClosable {
                fn from(value: &AudioGraph) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, super::super::Foundation::IClosable> for AudioGraph {
                fn into_param(self) -> ::windows::Param<'a, super::super::Foundation::IClosable> {
                    ::windows::Param::Owned(::std::convert::Into::<
                        super::super::Foundation::IClosable,
                    >::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, super::super::Foundation::IClosable> for &'a AudioGraph {
                fn into_param(self) -> ::windows::Param<'a, super::super::Foundation::IClosable> {
                    ::windows::Param::Owned(::std::convert::Into::<
                        super::super::Foundation::IClosable,
                    >::into(::std::clone::Clone::clone(
                        self,
                    )))
                }
            }
            unsafe impl ::std::marker::Send for AudioGraph {}
            unsafe impl ::std::marker::Sync for AudioGraph {}
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IAudioGraphSettings(::windows::Object);
            unsafe impl ::windows::Interface for IAudioGraphSettings {
                type Vtable = IAudioGraphSettings_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    492397695,
                    59134,
                    17960,
                    [132, 248, 157, 139, 219, 162, 87, 133],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAudioGraphSettings_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut super::Render::AudioRenderCategory,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: super::Render::AudioRenderCategory,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IAudioGraphSettings2(::windows::Object);
            unsafe impl ::windows::Interface for IAudioGraphSettings2 {
                type Vtable = IAudioGraphSettings2_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    1922144135,
                    19883,
                    18147,
                    [180, 201, 216, 225, 162, 99, 96, 98],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAudioGraphSettings2_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: f64,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut f64,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IAudioGraphSettingsFactory(::windows::Object);
            unsafe impl ::windows::Interface for IAudioGraphSettingsFactory {
                type Vtable = IAudioGraphSettingsFactory_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    2782469318,
                    49899,
                    19041,
                    [162, 20, 29, 102, 215, 95, 131, 218],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAudioGraphSettingsFactory_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    audiorendercategory: super::Render::AudioRenderCategory,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct AudioGraphSettings(::windows::Object);
            impl AudioGraphSettings {
                pub fn DesiredSamplesPerQuantum(&self) -> ::windows::Result<i32> {
                    let this = self;
                    unsafe {
                        let mut result__: <i32 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).12)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<i32>(result__)
                    }
                }
                pub fn SetDesiredSamplesPerQuantum(&self, value: i32) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).13)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn AudioRenderCategory(
                    &self,
                ) -> ::windows::Result<super::Render::AudioRenderCategory> {
                    let this = self;
                    unsafe {
                        let mut result__ : < super :: Render :: AudioRenderCategory as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                        (::windows::Interface::vtable(this).14)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::Render::AudioRenderCategory>(result__)
                    }
                }
                pub fn SetAudioRenderCategory(
                    &self,
                    value: super::Render::AudioRenderCategory,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).15)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn Create(
                    audiorendercategory: super::Render::AudioRenderCategory,
                ) -> ::windows::Result<AudioGraphSettings> {
                    Self::IAudioGraphSettingsFactory(|this| unsafe {
                        let mut result__: <AudioGraphSettings as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            audiorendercategory,
                            &mut result__,
                        )
                        .from_abi::<AudioGraphSettings>(result__)
                    })
                }
                pub fn SetMaxPlaybackSpeedFactor(&self, value: f64) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioGraphSettings2>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn MaxPlaybackSpeedFactor(&self) -> ::windows::Result<f64> {
                    let this = &::windows::Interface::cast::<IAudioGraphSettings2>(self).unwrap();
                    unsafe {
                        let mut result__: <f64 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<f64>(result__)
                    }
                }
                fn IAudioGraphSettingsFactory<
                    R,
                    F: FnOnce(&IAudioGraphSettingsFactory) -> ::windows::Result<R>,
                >(
                    callback: F,
                ) -> ::windows::Result<R> {
                    static mut SHARED: ::windows::FactoryCache<
                        AudioGraphSettings,
                        IAudioGraphSettingsFactory,
                    > = ::windows::FactoryCache::new();
                    unsafe { SHARED.call(callback) }
                }
            }
            unsafe impl ::windows::RuntimeType for AudioGraphSettings {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.AudioGraphSettings;{1d59647f-e6fe-4628-84f8-9d8bdba25785})" ) ;
            }
            unsafe impl ::windows::Interface for AudioGraphSettings {
                type Vtable = IAudioGraphSettings_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    492397695,
                    59134,
                    17960,
                    [132, 248, 157, 139, 219, 162, 87, 133],
                );
            }
            impl ::windows::RuntimeName for AudioGraphSettings {
                const NAME: &'static str = "Windows.Media.Audio.AudioGraphSettings";
            }
            impl ::std::convert::From<AudioGraphSettings> for ::windows::Object {
                fn from(value: AudioGraphSettings) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&AudioGraphSettings> for ::windows::Object {
                fn from(value: &AudioGraphSettings) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for AudioGraphSettings {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a AudioGraphSettings {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            unsafe impl ::std::marker::Send for AudioGraphSettings {}
            unsafe impl ::std::marker::Sync for AudioGraphSettings {}
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct ICreateAudioGraphResult(::windows::Object);
            unsafe impl ::windows::Interface for ICreateAudioGraphResult {
                type Vtable = ICreateAudioGraphResult_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    1414786942,
                    31710,
                    19318,
                    [187, 93, 72, 247, 156, 252, 140, 11],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ICreateAudioGraphResult_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct ICreateAudioGraphResult2(::windows::Object);
            unsafe impl ::windows::Interface for ICreateAudioGraphResult2 {
                type Vtable = ICreateAudioGraphResult2_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    1836289532,
                    35014,
                    20427,
                    [165, 52, 133, 206, 221, 64, 80, 161],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ICreateAudioGraphResult2_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::HRESULT,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct CreateAudioGraphResult(::windows::Object);
            impl CreateAudioGraphResult {
                pub fn Graph(&self) -> ::windows::Result<AudioGraph> {
                    let this = self;
                    unsafe {
                        let mut result__: <AudioGraph as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<AudioGraph>(result__)
                    }
                }
                pub fn ExtendedError(&self) -> ::windows::Result<::windows::HRESULT> {
                    let this =
                        &::windows::Interface::cast::<ICreateAudioGraphResult2>(self).unwrap();
                    unsafe {
                        let mut result__: <::windows::HRESULT as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HRESULT>(result__)
                    }
                }
            }
            unsafe impl ::windows::RuntimeType for CreateAudioGraphResult {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.CreateAudioGraphResult;{5453ef7e-7bde-4b76-bb5d-48f79cfc8c0b})" ) ;
            }
            unsafe impl ::windows::Interface for CreateAudioGraphResult {
                type Vtable = ICreateAudioGraphResult_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    1414786942,
                    31710,
                    19318,
                    [187, 93, 72, 247, 156, 252, 140, 11],
                );
            }
            impl ::windows::RuntimeName for CreateAudioGraphResult {
                const NAME: &'static str = "Windows.Media.Audio.CreateAudioGraphResult";
            }
            impl ::std::convert::From<CreateAudioGraphResult> for ::windows::Object {
                fn from(value: CreateAudioGraphResult) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&CreateAudioGraphResult> for ::windows::Object {
                fn from(value: &CreateAudioGraphResult) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for CreateAudioGraphResult {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a CreateAudioGraphResult {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            unsafe impl ::std::marker::Send for CreateAudioGraphResult {}
            unsafe impl ::std::marker::Sync for CreateAudioGraphResult {}
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct ICreateAudioDeviceOutputNodeResult(::windows::Object);
            unsafe impl ::windows::Interface for ICreateAudioDeviceOutputNodeResult {
                type Vtable = ICreateAudioDeviceOutputNodeResult_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    4151799079,
                    7578,
                    18423,
                    [156, 212, 40, 89, 204, 27, 123, 255],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ICreateAudioDeviceOutputNodeResult_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct ICreateAudioDeviceOutputNodeResult2(::windows::Object);
            unsafe impl ::windows::Interface for ICreateAudioDeviceOutputNodeResult2 {
                type Vtable = ICreateAudioDeviceOutputNodeResult2_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    1214523039,
                    48590,
                    19121,
                    [189, 56, 251, 174, 147, 174, 218, 202],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ICreateAudioDeviceOutputNodeResult2_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::HRESULT,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct CreateAudioDeviceOutputNodeResult(::windows::Object);
            impl CreateAudioDeviceOutputNodeResult {
                pub fn DeviceOutputNode(&self) -> ::windows::Result<AudioDeviceOutputNode> {
                    let this = self;
                    unsafe {
                        let mut result__: <AudioDeviceOutputNode as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<AudioDeviceOutputNode>(result__)
                    }
                }
                pub fn ExtendedError(&self) -> ::windows::Result<::windows::HRESULT> {
                    let this =
                        &::windows::Interface::cast::<ICreateAudioDeviceOutputNodeResult2>(self)
                            .unwrap();
                    unsafe {
                        let mut result__: <::windows::HRESULT as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HRESULT>(result__)
                    }
                }
            }
            unsafe impl ::windows::RuntimeType for CreateAudioDeviceOutputNodeResult {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.CreateAudioDeviceOutputNodeResult;{f7776d27-1d9a-47f7-9cd4-2859cc1b7bff})" ) ;
            }
            unsafe impl ::windows::Interface for CreateAudioDeviceOutputNodeResult {
                type Vtable = ICreateAudioDeviceOutputNodeResult_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    4151799079,
                    7578,
                    18423,
                    [156, 212, 40, 89, 204, 27, 123, 255],
                );
            }
            impl ::windows::RuntimeName for CreateAudioDeviceOutputNodeResult {
                const NAME: &'static str = "Windows.Media.Audio.CreateAudioDeviceOutputNodeResult";
            }
            impl ::std::convert::From<CreateAudioDeviceOutputNodeResult> for ::windows::Object {
                fn from(value: CreateAudioDeviceOutputNodeResult) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&CreateAudioDeviceOutputNodeResult> for ::windows::Object {
                fn from(value: &CreateAudioDeviceOutputNodeResult) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for CreateAudioDeviceOutputNodeResult {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a CreateAudioDeviceOutputNodeResult {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            unsafe impl ::std::marker::Send for CreateAudioDeviceOutputNodeResult {}
            unsafe impl ::std::marker::Sync for CreateAudioDeviceOutputNodeResult {}
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IAudioNode(::windows::Object);
            unsafe impl ::windows::Interface for IAudioNode {
                type Vtable = IAudioNode_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    356031871,
                    56280,
                    18457,
                    [191, 3, 102, 142, 147, 87, 205, 109],
                );
            }
            impl IAudioNode {
                pub fn SetOutgoingGain(&self, value: f64) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn OutgoingGain(&self) -> ::windows::Result<f64> {
                    let this = self;
                    unsafe {
                        let mut result__: <f64 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<f64>(result__)
                    }
                }
                pub fn ConsumeInput(&self) -> ::windows::Result<bool> {
                    let this = self;
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).10)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn SetConsumeInput(&self, value: bool) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).11)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn Start(&self) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).12)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Stop(&self) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).13)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Reset(&self) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).14)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Close(&self) -> ::windows::Result<()> {
                    let this =
                        &::windows::Interface::cast::<super::super::Foundation::IClosable>(self)
                            .unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this)).ok()
                    }
                }
            }
            unsafe impl ::windows::RuntimeType for IAudioNode {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer =
                    ::windows::ConstBuffer::from_slice(b"{15389d7f-dbd8-4819-bf03-668e9357cd6d}");
            }
            impl ::std::convert::From<IAudioNode> for ::windows::Object {
                fn from(value: IAudioNode) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&IAudioNode> for ::windows::Object {
                fn from(value: &IAudioNode) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IAudioNode {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IAudioNode {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl ::std::convert::From<IAudioNode> for super::super::Foundation::IClosable {
                fn from(value: IAudioNode) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&IAudioNode> for super::super::Foundation::IClosable {
                fn from(value: &IAudioNode) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, super::super::Foundation::IClosable> for IAudioNode {
                fn into_param(self) -> ::windows::Param<'a, super::super::Foundation::IClosable> {
                    ::windows::Param::Owned(::std::convert::Into::<
                        super::super::Foundation::IClosable,
                    >::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, super::super::Foundation::IClosable> for &'a IAudioNode {
                fn into_param(self) -> ::windows::Param<'a, super::super::Foundation::IClosable> {
                    ::windows::Param::Owned(::std::convert::Into::<
                        super::super::Foundation::IClosable,
                    >::into(::std::clone::Clone::clone(
                        self,
                    )))
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAudioNode_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: f64,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut f64,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut bool,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: bool,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IAudioDeviceOutputNode(::windows::Object);
            unsafe impl ::windows::Interface for IAudioDeviceOutputNode {
                type Vtable = IAudioDeviceOutputNode_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    909040639,
                    65308,
                    17460,
                    [158, 15, 189, 46, 245, 34, 172, 130],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAudioDeviceOutputNode_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IAudioNodeWithListener(::windows::Object);
            unsafe impl ::windows::Interface for IAudioNodeWithListener {
                type Vtable = IAudioNodeWithListener_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    235901052,
                    31231,
                    17732,
                    [158, 235, 1, 37, 123, 21, 16, 90],
                );
            }
            impl IAudioNodeWithListener {
                pub fn Close(&self) -> ::windows::Result<()> {
                    let this =
                        &::windows::Interface::cast::<super::super::Foundation::IClosable>(self)
                            .unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn SetOutgoingGain(&self, value: f64) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn OutgoingGain(&self) -> ::windows::Result<f64> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        let mut result__: <f64 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<f64>(result__)
                    }
                }
                pub fn ConsumeInput(&self) -> ::windows::Result<bool> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).10)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn SetConsumeInput(&self, value: bool) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).11)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn Start(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).12)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Stop(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).13)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Reset(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).14)(::windows::Abi::abi(this)).ok()
                    }
                }
            }
            unsafe impl ::windows::RuntimeType for IAudioNodeWithListener {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer =
                    ::windows::ConstBuffer::from_slice(b"{0e0f907c-79ff-4544-9eeb-01257b15105a}");
            }
            impl ::std::convert::From<IAudioNodeWithListener> for ::windows::Object {
                fn from(value: IAudioNodeWithListener) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&IAudioNodeWithListener> for ::windows::Object {
                fn from(value: &IAudioNodeWithListener) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IAudioNodeWithListener {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IAudioNodeWithListener {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl ::std::convert::From<IAudioNodeWithListener> for super::super::Foundation::IClosable {
                fn from(value: IAudioNodeWithListener) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&IAudioNodeWithListener> for super::super::Foundation::IClosable {
                fn from(value: &IAudioNodeWithListener) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, super::super::Foundation::IClosable> for IAudioNodeWithListener {
                fn into_param(self) -> ::windows::Param<'a, super::super::Foundation::IClosable> {
                    ::windows::Param::Owned(::std::convert::Into::<
                        super::super::Foundation::IClosable,
                    >::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, super::super::Foundation::IClosable>
                for &'a IAudioNodeWithListener
            {
                fn into_param(self) -> ::windows::Param<'a, super::super::Foundation::IClosable> {
                    ::windows::Param::Owned(::std::convert::Into::<
                        super::super::Foundation::IClosable,
                    >::into(::std::clone::Clone::clone(
                        self,
                    )))
                }
            }
            impl ::std::convert::From<IAudioNodeWithListener> for IAudioNode {
                fn from(value: IAudioNodeWithListener) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&IAudioNodeWithListener> for IAudioNode {
                fn from(value: &IAudioNodeWithListener) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioNode> for IAudioNodeWithListener {
                fn into_param(self) -> ::windows::Param<'a, IAudioNode> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioNode>::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioNode> for &'a IAudioNodeWithListener {
                fn into_param(self) -> ::windows::Param<'a, IAudioNode> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioNode>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAudioNodeWithListener_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct AudioDeviceOutputNode(::windows::Object);
            impl AudioDeviceOutputNode {
                pub fn Close(&self) -> ::windows::Result<()> {
                    let this =
                        &::windows::Interface::cast::<super::super::Foundation::IClosable>(self)
                            .unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn SetOutgoingGain(&self, value: f64) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn OutgoingGain(&self) -> ::windows::Result<f64> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        let mut result__: <f64 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<f64>(result__)
                    }
                }
                pub fn ConsumeInput(&self) -> ::windows::Result<bool> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).10)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn SetConsumeInput(&self, value: bool) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).11)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn Start(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).12)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Stop(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).13)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Reset(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).14)(::windows::Abi::abi(this)).ok()
                    }
                }
            }
            unsafe impl ::windows::RuntimeType for AudioDeviceOutputNode {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.AudioDeviceOutputNode;{362edbff-ff1c-4434-9e0f-bd2ef522ac82})" ) ;
            }
            unsafe impl ::windows::Interface for AudioDeviceOutputNode {
                type Vtable = IAudioDeviceOutputNode_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    909040639,
                    65308,
                    17460,
                    [158, 15, 189, 46, 245, 34, 172, 130],
                );
            }
            impl ::windows::RuntimeName for AudioDeviceOutputNode {
                const NAME: &'static str = "Windows.Media.Audio.AudioDeviceOutputNode";
            }
            impl ::std::convert::From<AudioDeviceOutputNode> for ::windows::Object {
                fn from(value: AudioDeviceOutputNode) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&AudioDeviceOutputNode> for ::windows::Object {
                fn from(value: &AudioDeviceOutputNode) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for AudioDeviceOutputNode {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a AudioDeviceOutputNode {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl ::std::convert::From<AudioDeviceOutputNode> for super::super::Foundation::IClosable {
                fn from(value: AudioDeviceOutputNode) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&AudioDeviceOutputNode> for super::super::Foundation::IClosable {
                fn from(value: &AudioDeviceOutputNode) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, super::super::Foundation::IClosable> for AudioDeviceOutputNode {
                fn into_param(self) -> ::windows::Param<'a, super::super::Foundation::IClosable> {
                    ::windows::Param::Owned(::std::convert::Into::<
                        super::super::Foundation::IClosable,
                    >::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, super::super::Foundation::IClosable>
                for &'a AudioDeviceOutputNode
            {
                fn into_param(self) -> ::windows::Param<'a, super::super::Foundation::IClosable> {
                    ::windows::Param::Owned(::std::convert::Into::<
                        super::super::Foundation::IClosable,
                    >::into(::std::clone::Clone::clone(
                        self,
                    )))
                }
            }
            impl ::std::convert::From<AudioDeviceOutputNode> for IAudioNode {
                fn from(value: AudioDeviceOutputNode) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&AudioDeviceOutputNode> for IAudioNode {
                fn from(value: &AudioDeviceOutputNode) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioNode> for AudioDeviceOutputNode {
                fn into_param(self) -> ::windows::Param<'a, IAudioNode> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioNode>::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioNode> for &'a AudioDeviceOutputNode {
                fn into_param(self) -> ::windows::Param<'a, IAudioNode> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioNode>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            impl ::std::convert::From<AudioDeviceOutputNode> for IAudioNodeWithListener {
                fn from(value: AudioDeviceOutputNode) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&AudioDeviceOutputNode> for IAudioNodeWithListener {
                fn from(value: &AudioDeviceOutputNode) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioNodeWithListener> for AudioDeviceOutputNode {
                fn into_param(self) -> ::windows::Param<'a, IAudioNodeWithListener> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioNodeWithListener>::into(
                        self,
                    ))
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioNodeWithListener> for &'a AudioDeviceOutputNode {
                fn into_param(self) -> ::windows::Param<'a, IAudioNodeWithListener> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioNodeWithListener>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            unsafe impl ::std::marker::Send for AudioDeviceOutputNode {}
            unsafe impl ::std::marker::Sync for AudioDeviceOutputNode {}
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct ICreateAudioFileInputNodeResult(::windows::Object);
            unsafe impl ::windows::Interface for ICreateAudioFileInputNodeResult {
                type Vtable = ICreateAudioFileInputNodeResult_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    3464746524,
                    58007,
                    19536,
                    [156, 231, 28, 122, 105, 214, 189, 9],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ICreateAudioFileInputNodeResult_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct ICreateAudioFileInputNodeResult2(::windows::Object);
            unsafe impl ::windows::Interface for ICreateAudioFileInputNodeResult2 {
                type Vtable = ICreateAudioFileInputNodeResult2_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    4178059296,
                    15744,
                    20448,
                    [129, 193, 118, 143, 234, 124, 167, 224],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ICreateAudioFileInputNodeResult2_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::HRESULT,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct CreateAudioFileInputNodeResult(::windows::Object);
            impl CreateAudioFileInputNodeResult {
                pub fn FileInputNode(&self) -> ::windows::Result<AudioFileInputNode> {
                    let this = self;
                    unsafe {
                        let mut result__: <AudioFileInputNode as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<AudioFileInputNode>(result__)
                    }
                }
                pub fn ExtendedError(&self) -> ::windows::Result<::windows::HRESULT> {
                    let this =
                        &::windows::Interface::cast::<ICreateAudioFileInputNodeResult2>(self)
                            .unwrap();
                    unsafe {
                        let mut result__: <::windows::HRESULT as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HRESULT>(result__)
                    }
                }
            }
            unsafe impl ::windows::RuntimeType for CreateAudioFileInputNodeResult {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.CreateAudioFileInputNodeResult;{ce83d61c-e297-4c50-9ce7-1c7a69d6bd09})" ) ;
            }
            unsafe impl ::windows::Interface for CreateAudioFileInputNodeResult {
                type Vtable = ICreateAudioFileInputNodeResult_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    3464746524,
                    58007,
                    19536,
                    [156, 231, 28, 122, 105, 214, 189, 9],
                );
            }
            impl ::windows::RuntimeName for CreateAudioFileInputNodeResult {
                const NAME: &'static str = "Windows.Media.Audio.CreateAudioFileInputNodeResult";
            }
            impl ::std::convert::From<CreateAudioFileInputNodeResult> for ::windows::Object {
                fn from(value: CreateAudioFileInputNodeResult) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&CreateAudioFileInputNodeResult> for ::windows::Object {
                fn from(value: &CreateAudioFileInputNodeResult) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for CreateAudioFileInputNodeResult {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a CreateAudioFileInputNodeResult {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            unsafe impl ::std::marker::Send for CreateAudioFileInputNodeResult {}
            unsafe impl ::std::marker::Sync for CreateAudioFileInputNodeResult {}
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IAudioInputNode(::windows::Object);
            unsafe impl ::windows::Interface for IAudioInputNode {
                type Vtable = IAudioInputNode_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    3511156828,
                    33832,
                    18308,
                    [183, 253, 169, 157, 70, 140, 93, 32],
                );
            }
            impl IAudioInputNode {
                pub fn AddOutgoingConnection<'a>(
                    &self,
                    destination: impl ::windows::IntoParam<'a, IAudioNode>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            destination.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn AddOutgoingConnectionWithGain<'a>(
                    &self,
                    destination: impl ::windows::IntoParam<'a, IAudioNode>,
                    gain: f64,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            destination.into_param().abi(),
                            gain,
                        )
                        .ok()
                    }
                }
                pub fn RemoveOutgoingConnection<'a>(
                    &self,
                    destination: impl ::windows::IntoParam<'a, IAudioNode>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            destination.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Close(&self) -> ::windows::Result<()> {
                    let this =
                        &::windows::Interface::cast::<super::super::Foundation::IClosable>(self)
                            .unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn SetOutgoingGain(&self, value: f64) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn OutgoingGain(&self) -> ::windows::Result<f64> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        let mut result__: <f64 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<f64>(result__)
                    }
                }
                pub fn ConsumeInput(&self) -> ::windows::Result<bool> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).10)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn SetConsumeInput(&self, value: bool) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).11)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn Start(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).12)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Stop(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).13)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Reset(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).14)(::windows::Abi::abi(this)).ok()
                    }
                }
            }
            unsafe impl ::windows::RuntimeType for IAudioInputNode {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer =
                    ::windows::ConstBuffer::from_slice(b"{d148005c-8428-4784-b7fd-a99d468c5d20}");
            }
            impl ::std::convert::From<IAudioInputNode> for ::windows::Object {
                fn from(value: IAudioInputNode) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&IAudioInputNode> for ::windows::Object {
                fn from(value: &IAudioInputNode) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IAudioInputNode {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IAudioInputNode {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl ::std::convert::From<IAudioInputNode> for super::super::Foundation::IClosable {
                fn from(value: IAudioInputNode) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&IAudioInputNode> for super::super::Foundation::IClosable {
                fn from(value: &IAudioInputNode) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, super::super::Foundation::IClosable> for IAudioInputNode {
                fn into_param(self) -> ::windows::Param<'a, super::super::Foundation::IClosable> {
                    ::windows::Param::Owned(::std::convert::Into::<
                        super::super::Foundation::IClosable,
                    >::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, super::super::Foundation::IClosable> for &'a IAudioInputNode {
                fn into_param(self) -> ::windows::Param<'a, super::super::Foundation::IClosable> {
                    ::windows::Param::Owned(::std::convert::Into::<
                        super::super::Foundation::IClosable,
                    >::into(::std::clone::Clone::clone(
                        self,
                    )))
                }
            }
            impl ::std::convert::From<IAudioInputNode> for IAudioNode {
                fn from(value: IAudioInputNode) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&IAudioInputNode> for IAudioNode {
                fn from(value: &IAudioInputNode) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioNode> for IAudioInputNode {
                fn into_param(self) -> ::windows::Param<'a, IAudioNode> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioNode>::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioNode> for &'a IAudioInputNode {
                fn into_param(self) -> ::windows::Param<'a, IAudioNode> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioNode>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAudioInputNode_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    destination: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    destination: ::windows::RawPtr,
                    gain: f64,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    destination: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IAudioFileInputNode(::windows::Object);
            unsafe impl ::windows::Interface for IAudioFileInputNode {
                type Vtable = IAudioFileInputNode_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    2421909448,
                    28517,
                    19668,
                    [136, 144, 70, 148, 132, 60, 39, 109],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAudioFileInputNode_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: f64,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut f64,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut super::super::Foundation::TimeSpan,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    position: super::super::Foundation::TimeSpan,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut super::super::Foundation::TimeSpan,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IAudioInputNode2(::windows::Object);
            unsafe impl ::windows::Interface for IAudioInputNode2 {
                type Vtable = IAudioInputNode2_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    2421249719,
                    51816,
                    19565,
                    [168, 188, 227, 238, 23, 254, 63, 210],
                );
            }
            impl IAudioInputNode2 {
                pub fn Close(&self) -> ::windows::Result<()> {
                    let this =
                        &::windows::Interface::cast::<super::super::Foundation::IClosable>(self)
                            .unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn AddOutgoingConnection<'a>(
                    &self,
                    destination: impl ::windows::IntoParam<'a, IAudioNode>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioInputNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            destination.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn AddOutgoingConnectionWithGain<'a>(
                    &self,
                    destination: impl ::windows::IntoParam<'a, IAudioNode>,
                    gain: f64,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioInputNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            destination.into_param().abi(),
                            gain,
                        )
                        .ok()
                    }
                }
                pub fn RemoveOutgoingConnection<'a>(
                    &self,
                    destination: impl ::windows::IntoParam<'a, IAudioNode>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioInputNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            destination.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn SetOutgoingGain(&self, value: f64) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn OutgoingGain(&self) -> ::windows::Result<f64> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        let mut result__: <f64 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<f64>(result__)
                    }
                }
                pub fn ConsumeInput(&self) -> ::windows::Result<bool> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).10)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn SetConsumeInput(&self, value: bool) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).11)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn Start(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).12)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Stop(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).13)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Reset(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).14)(::windows::Abi::abi(this)).ok()
                    }
                }
            }
            unsafe impl ::windows::RuntimeType for IAudioInputNode2 {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer =
                    ::windows::ConstBuffer::from_slice(b"{905156b7-ca68-4c6d-a8bc-e3ee17fe3fd2}");
            }
            impl ::std::convert::From<IAudioInputNode2> for ::windows::Object {
                fn from(value: IAudioInputNode2) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&IAudioInputNode2> for ::windows::Object {
                fn from(value: &IAudioInputNode2) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IAudioInputNode2 {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IAudioInputNode2 {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl ::std::convert::From<IAudioInputNode2> for super::super::Foundation::IClosable {
                fn from(value: IAudioInputNode2) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&IAudioInputNode2> for super::super::Foundation::IClosable {
                fn from(value: &IAudioInputNode2) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, super::super::Foundation::IClosable> for IAudioInputNode2 {
                fn into_param(self) -> ::windows::Param<'a, super::super::Foundation::IClosable> {
                    ::windows::Param::Owned(::std::convert::Into::<
                        super::super::Foundation::IClosable,
                    >::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, super::super::Foundation::IClosable> for &'a IAudioInputNode2 {
                fn into_param(self) -> ::windows::Param<'a, super::super::Foundation::IClosable> {
                    ::windows::Param::Owned(::std::convert::Into::<
                        super::super::Foundation::IClosable,
                    >::into(::std::clone::Clone::clone(
                        self,
                    )))
                }
            }
            impl ::std::convert::From<IAudioInputNode2> for IAudioInputNode {
                fn from(value: IAudioInputNode2) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&IAudioInputNode2> for IAudioInputNode {
                fn from(value: &IAudioInputNode2) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioInputNode> for IAudioInputNode2 {
                fn into_param(self) -> ::windows::Param<'a, IAudioInputNode> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioInputNode>::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioInputNode> for &'a IAudioInputNode2 {
                fn into_param(self) -> ::windows::Param<'a, IAudioInputNode> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioInputNode>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            impl ::std::convert::From<IAudioInputNode2> for IAudioNode {
                fn from(value: IAudioInputNode2) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&IAudioInputNode2> for IAudioNode {
                fn from(value: &IAudioInputNode2) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioNode> for IAudioInputNode2 {
                fn into_param(self) -> ::windows::Param<'a, IAudioNode> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioNode>::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioNode> for &'a IAudioInputNode2 {
                fn into_param(self) -> ::windows::Param<'a, IAudioNode> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioNode>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAudioInputNode2_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct AudioFileInputNode(::windows::Object);
            impl AudioFileInputNode {
                pub fn SetPlaybackSpeedFactor(&self, value: f64) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn PlaybackSpeedFactor(&self) -> ::windows::Result<f64> {
                    let this = self;
                    unsafe {
                        let mut result__: <f64 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<f64>(result__)
                    }
                }
                pub fn Position(&self) -> ::windows::Result<super::super::Foundation::TimeSpan> {
                    let this = self;
                    unsafe {
                        let mut result__ : < super :: super :: Foundation :: TimeSpan as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::TimeSpan>(result__)
                    }
                }
                pub fn Seek<'a>(
                    &self,
                    position: impl ::windows::IntoParam<'a, super::super::Foundation::TimeSpan>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            position.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn Duration(&self) -> ::windows::Result<super::super::Foundation::TimeSpan> {
                    let this = self;
                    unsafe {
                        let mut result__ : < super :: super :: Foundation :: TimeSpan as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                        (::windows::Interface::vtable(this).16)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::TimeSpan>(result__)
                    }
                }
                pub fn SourceFile(&self) -> ::windows::Result<super::super::Storage::StorageFile> {
                    let this = self;
                    unsafe {
                        let mut result__ : < super :: super :: Storage :: StorageFile as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                        (::windows::Interface::vtable(this).17)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Storage::StorageFile>(result__)
                    }
                }
                pub fn Close(&self) -> ::windows::Result<()> {
                    let this =
                        &::windows::Interface::cast::<super::super::Foundation::IClosable>(self)
                            .unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn AddOutgoingConnection<'a>(
                    &self,
                    destination: impl ::windows::IntoParam<'a, IAudioNode>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioInputNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).7)(
                            ::windows::Abi::abi(this),
                            destination.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn AddOutgoingConnectionWithGain<'a>(
                    &self,
                    destination: impl ::windows::IntoParam<'a, IAudioNode>,
                    gain: f64,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioInputNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            destination.into_param().abi(),
                            gain,
                        )
                        .ok()
                    }
                }
                pub fn RemoveOutgoingConnection<'a>(
                    &self,
                    destination: impl ::windows::IntoParam<'a, IAudioNode>,
                ) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioInputNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            destination.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn SetOutgoingGain(&self, value: f64) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn OutgoingGain(&self) -> ::windows::Result<f64> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        let mut result__: <f64 as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<f64>(result__)
                    }
                }
                pub fn ConsumeInput(&self) -> ::windows::Result<bool> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).10)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<bool>(result__)
                    }
                }
                pub fn SetConsumeInput(&self, value: bool) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).11)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn Start(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).12)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Stop(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).13)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn Reset(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IAudioNode>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).14)(::windows::Abi::abi(this)).ok()
                    }
                }
            }
            unsafe impl ::windows::RuntimeType for AudioFileInputNode {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Media.Audio.AudioFileInputNode;{905b67c8-6f65-4cd4-8890-4694843c276d})" ) ;
            }
            unsafe impl ::windows::Interface for AudioFileInputNode {
                type Vtable = IAudioFileInputNode_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    2421909448,
                    28517,
                    19668,
                    [136, 144, 70, 148, 132, 60, 39, 109],
                );
            }
            impl ::windows::RuntimeName for AudioFileInputNode {
                const NAME: &'static str = "Windows.Media.Audio.AudioFileInputNode";
            }
            impl ::std::convert::From<AudioFileInputNode> for ::windows::Object {
                fn from(value: AudioFileInputNode) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&AudioFileInputNode> for ::windows::Object {
                fn from(value: &AudioFileInputNode) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for AudioFileInputNode {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a AudioFileInputNode {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            impl ::std::convert::From<AudioFileInputNode> for super::super::Foundation::IClosable {
                fn from(value: AudioFileInputNode) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&AudioFileInputNode> for super::super::Foundation::IClosable {
                fn from(value: &AudioFileInputNode) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, super::super::Foundation::IClosable> for AudioFileInputNode {
                fn into_param(self) -> ::windows::Param<'a, super::super::Foundation::IClosable> {
                    ::windows::Param::Owned(::std::convert::Into::<
                        super::super::Foundation::IClosable,
                    >::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, super::super::Foundation::IClosable> for &'a AudioFileInputNode {
                fn into_param(self) -> ::windows::Param<'a, super::super::Foundation::IClosable> {
                    ::windows::Param::Owned(::std::convert::Into::<
                        super::super::Foundation::IClosable,
                    >::into(::std::clone::Clone::clone(
                        self,
                    )))
                }
            }
            impl ::std::convert::From<AudioFileInputNode> for IAudioInputNode {
                fn from(value: AudioFileInputNode) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&AudioFileInputNode> for IAudioInputNode {
                fn from(value: &AudioFileInputNode) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioInputNode> for AudioFileInputNode {
                fn into_param(self) -> ::windows::Param<'a, IAudioInputNode> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioInputNode>::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioInputNode> for &'a AudioFileInputNode {
                fn into_param(self) -> ::windows::Param<'a, IAudioInputNode> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioInputNode>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            impl ::std::convert::From<AudioFileInputNode> for IAudioNode {
                fn from(value: AudioFileInputNode) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&AudioFileInputNode> for IAudioNode {
                fn from(value: &AudioFileInputNode) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioNode> for AudioFileInputNode {
                fn into_param(self) -> ::windows::Param<'a, IAudioNode> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioNode>::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioNode> for &'a AudioFileInputNode {
                fn into_param(self) -> ::windows::Param<'a, IAudioNode> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioNode>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            impl ::std::convert::From<AudioFileInputNode> for IAudioInputNode2 {
                fn from(value: AudioFileInputNode) -> Self {
                    ::std::convert::From::from(&value)
                }
            }
            impl ::std::convert::From<&AudioFileInputNode> for IAudioInputNode2 {
                fn from(value: &AudioFileInputNode) -> Self {
                    ::windows::Interface::cast(value).unwrap()
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioInputNode2> for AudioFileInputNode {
                fn into_param(self) -> ::windows::Param<'a, IAudioInputNode2> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioInputNode2>::into(self))
                }
            }
            impl<'a> ::windows::IntoParam<'a, IAudioInputNode2> for &'a AudioFileInputNode {
                fn into_param(self) -> ::windows::Param<'a, IAudioInputNode2> {
                    ::windows::Param::Owned(::std::convert::Into::<IAudioInputNode2>::into(
                        ::std::clone::Clone::clone(self),
                    ))
                }
            }
            unsafe impl ::std::marker::Send for AudioFileInputNode {}
            unsafe impl ::std::marker::Sync for AudioFileInputNode {}
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Render {
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: marker :: Copy,
                :: std :: clone :: Clone,
                :: std :: default :: Default,
                :: std :: fmt :: Debug,
            )]
            #[repr(transparent)]
            pub struct AudioRenderCategory(pub i32);
            impl AudioRenderCategory {
                pub const Other: Self = Self(0i32);
                pub const ForegroundOnlyMedia: Self = Self(1i32);
                pub const BackgroundCapableMedia: Self = Self(2i32);
                pub const Communications: Self = Self(3i32);
                pub const Alerts: Self = Self(4i32);
                pub const SoundEffects: Self = Self(5i32);
                pub const GameEffects: Self = Self(6i32);
                pub const GameMedia: Self = Self(7i32);
                pub const GameChat: Self = Self(8i32);
                pub const Speech: Self = Self(9i32);
                pub const Movie: Self = Self(10i32);
                pub const Media: Self = Self(11i32);
            }
            impl ::std::convert::From<i32> for AudioRenderCategory {
                fn from(value: i32) -> Self {
                    Self(value)
                }
            }
            unsafe impl ::windows::Abi for AudioRenderCategory {
                type Abi = Self;
            }
            unsafe impl ::windows::RuntimeType for AudioRenderCategory {
                type DefaultType = Self;
                const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                    b"enum(Windows.Media.Render.AudioRenderCategory;i4)",
                );
            }
        }
    }
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Storage {
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IStorageItem(::windows::Object);
        unsafe impl ::windows::Interface for IStorageItem {
            type Vtable = IStorageItem_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                1107798422,
                51759,
                17143,
                [189, 232, 139, 16, 69, 122, 127, 48],
            );
        }
        impl IStorageItem {
            pub fn Name(&self) -> ::windows::Result<::windows::HString> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn Path(&self) -> ::windows::Result<::windows::HString> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).12)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
        }
        unsafe impl ::windows::RuntimeType for IStorageItem {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{4207a996-ca2f-42f7-bde8-8b10457a7f30}");
        }
        impl ::std::convert::From<IStorageItem> for ::windows::Object {
            fn from(value: IStorageItem) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IStorageItem> for ::windows::Object {
            fn from(value: &IStorageItem) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IStorageItem {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IStorageItem {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStorageItem_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IStorageFile(::windows::Object);
        unsafe impl ::windows::Interface for IStorageFile {
            type Vtable = IStorageFile_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                4198457734,
                16916,
                17036,
                [166, 76, 20, 201, 172, 115, 21, 234],
            );
        }
        impl IStorageFile {
            pub fn FileType(&self) -> ::windows::Result<::windows::HString> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn ContentType(&self) -> ::windows::Result<::windows::HString> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn CopyOverloadDefaultNameAndOptions<'a>(
                &self,
                destinationfolder: impl ::windows::IntoParam<'a, IStorageFolder>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFile>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFile > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).10)(
                        ::windows::Abi::abi(this),
                        destinationfolder.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
                }
            }
            pub fn CopyOverloadDefaultOptions<'a>(
                &self,
                destinationfolder: impl ::windows::IntoParam<'a, IStorageFolder>,
                desirednewname: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFile>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFile > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        destinationfolder.into_param().abi(),
                        desirednewname.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
                }
            }
            pub fn Name(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItem>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn Path(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItem>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).12)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
        }
        unsafe impl ::windows::RuntimeType for IStorageFile {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{fa3f6186-4214-428c-a64c-14c9ac7315ea}");
        }
        impl ::std::convert::From<IStorageFile> for ::windows::Object {
            fn from(value: IStorageFile) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IStorageFile> for ::windows::Object {
            fn from(value: &IStorageFile) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IStorageFile {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IStorageFile {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        impl ::std::convert::From<IStorageFile> for IStorageItem {
            fn from(value: IStorageFile) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&IStorageFile> for IStorageItem {
            fn from(value: &IStorageFile) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItem> for IStorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageItem> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItem>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItem> for &'a IStorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageItem> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItem>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<IStorageFile> for Streams::IInputStreamReference {
            fn from(value: IStorageFile) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&IStorageFile> for Streams::IInputStreamReference {
            fn from(value: &IStorageFile) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, Streams::IInputStreamReference> for IStorageFile {
            fn into_param(self) -> ::windows::Param<'a, Streams::IInputStreamReference> {
                ::windows::Param::Owned(
                    ::std::convert::Into::<Streams::IInputStreamReference>::into(self),
                )
            }
        }
        impl<'a> ::windows::IntoParam<'a, Streams::IInputStreamReference> for &'a IStorageFile {
            fn into_param(self) -> ::windows::Param<'a, Streams::IInputStreamReference> {
                ::windows::Param::Owned(
                    ::std::convert::Into::<Streams::IInputStreamReference>::into(
                        ::std::clone::Clone::clone(self),
                    ),
                )
            }
        }
        impl ::std::convert::From<IStorageFile> for Streams::IRandomAccessStreamReference {
            fn from(value: IStorageFile) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&IStorageFile> for Streams::IRandomAccessStreamReference {
            fn from(value: &IStorageFile) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, Streams::IRandomAccessStreamReference> for IStorageFile {
            fn into_param(self) -> ::windows::Param<'a, Streams::IRandomAccessStreamReference> {
                ::windows::Param::Owned(
                    ::std::convert::Into::<Streams::IRandomAccessStreamReference>::into(self),
                )
            }
        }
        impl<'a> ::windows::IntoParam<'a, Streams::IRandomAccessStreamReference> for &'a IStorageFile {
            fn into_param(self) -> ::windows::Param<'a, Streams::IRandomAccessStreamReference> {
                ::windows::Param::Owned(
                    ::std::convert::Into::<Streams::IRandomAccessStreamReference>::into(
                        ::std::clone::Clone::clone(self),
                    ),
                )
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStorageFile_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                destinationfolder: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                destinationfolder: ::windows::RawPtr,
                desirednewname: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IStorageItemProperties(::windows::Object);
        unsafe impl ::windows::Interface for IStorageItemProperties {
            type Vtable = IStorageItemProperties_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                2254849144,
                32809,
                18174,
                [167, 137, 28, 47, 62, 47, 251, 92],
            );
        }
        impl IStorageItemProperties {
            pub fn DisplayName(&self) -> ::windows::Result<::windows::HString> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).9)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn DisplayType(&self) -> ::windows::Result<::windows::HString> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).10)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn FolderRelativeId(&self) -> ::windows::Result<::windows::HString> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
        }
        unsafe impl ::windows::RuntimeType for IStorageItemProperties {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{86664478-8029-46fe-a789-1c2f3e2ffb5c}");
        }
        impl ::std::convert::From<IStorageItemProperties> for ::windows::Object {
            fn from(value: IStorageItemProperties) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IStorageItemProperties> for ::windows::Object {
            fn from(value: &IStorageItemProperties) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IStorageItemProperties {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IStorageItemProperties {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStorageItemProperties_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IStorageItemProperties2(::windows::Object);
        unsafe impl ::windows::Interface for IStorageItemProperties2 {
            type Vtable = IStorageItemProperties2_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                2391189841,
                1209,
                19410,
                [146, 157, 254, 243, 247, 22, 33, 208],
            );
        }
        impl IStorageItemProperties2 {
            pub fn DisplayName(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItemProperties>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).9)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn DisplayType(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItemProperties>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).10)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn FolderRelativeId(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItemProperties>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
        }
        unsafe impl ::windows::RuntimeType for IStorageItemProperties2 {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{8e86a951-04b9-4bd2-929d-fef3f71621d0}");
        }
        impl ::std::convert::From<IStorageItemProperties2> for ::windows::Object {
            fn from(value: IStorageItemProperties2) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IStorageItemProperties2> for ::windows::Object {
            fn from(value: &IStorageItemProperties2) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IStorageItemProperties2 {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IStorageItemProperties2 {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        impl ::std::convert::From<IStorageItemProperties2> for IStorageItemProperties {
            fn from(value: IStorageItemProperties2) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&IStorageItemProperties2> for IStorageItemProperties {
            fn from(value: &IStorageItemProperties2) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemProperties> for IStorageItemProperties2 {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemProperties> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItemProperties>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemProperties> for &'a IStorageItemProperties2 {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemProperties> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItemProperties>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStorageItemProperties2_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IStorageItem2(::windows::Object);
        unsafe impl ::windows::Interface for IStorageItem2 {
            type Vtable = IStorageItem2_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                1408837330,
                2108,
                17027,
                [180, 91, 129, 192, 7, 35, 126, 68],
            );
        }
        impl IStorageItem2 {
            pub fn GetParentAsync(
                &self,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFolder > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
                }
            }
            pub fn IsEqual<'a>(
                &self,
                item: impl ::windows::IntoParam<'a, IStorageItem>,
            ) -> ::windows::Result<bool> {
                let this = self;
                unsafe {
                    let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::windows::Abi::abi(this),
                        item.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<bool>(result__)
                }
            }
            pub fn Name(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItem>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn Path(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItem>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).12)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
        }
        unsafe impl ::windows::RuntimeType for IStorageItem2 {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{53f926d2-083c-4283-b45b-81c007237e44}");
        }
        impl ::std::convert::From<IStorageItem2> for ::windows::Object {
            fn from(value: IStorageItem2) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IStorageItem2> for ::windows::Object {
            fn from(value: &IStorageItem2) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IStorageItem2 {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IStorageItem2 {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        impl ::std::convert::From<IStorageItem2> for IStorageItem {
            fn from(value: IStorageItem2) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&IStorageItem2> for IStorageItem {
            fn from(value: &IStorageItem2) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItem> for IStorageItem2 {
            fn into_param(self) -> ::windows::Param<'a, IStorageItem> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItem>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItem> for &'a IStorageItem2 {
            fn into_param(self) -> ::windows::Param<'a, IStorageItem> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItem>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStorageItem2_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                item: ::windows::RawPtr,
                result__: *mut bool,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IStorageItemPropertiesWithProvider(::windows::Object);
        unsafe impl ::windows::Interface for IStorageItemPropertiesWithProvider {
            type Vtable = IStorageItemPropertiesWithProvider_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                2249978779,
                25448,
                19950,
                [180, 14, 116, 104, 74, 92, 231, 20],
            );
        }
        impl IStorageItemPropertiesWithProvider {
            pub fn DisplayName(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItemProperties>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).9)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn DisplayType(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItemProperties>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).10)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn FolderRelativeId(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItemProperties>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
        }
        unsafe impl ::windows::RuntimeType for IStorageItemPropertiesWithProvider {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{861bf39b-6368-4dee-b40e-74684a5ce714}");
        }
        impl ::std::convert::From<IStorageItemPropertiesWithProvider> for ::windows::Object {
            fn from(value: IStorageItemPropertiesWithProvider) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IStorageItemPropertiesWithProvider> for ::windows::Object {
            fn from(value: &IStorageItemPropertiesWithProvider) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IStorageItemPropertiesWithProvider {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IStorageItemPropertiesWithProvider {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        impl ::std::convert::From<IStorageItemPropertiesWithProvider> for IStorageItemProperties {
            fn from(value: IStorageItemPropertiesWithProvider) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&IStorageItemPropertiesWithProvider> for IStorageItemProperties {
            fn from(value: &IStorageItemPropertiesWithProvider) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemProperties> for IStorageItemPropertiesWithProvider {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemProperties> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItemProperties>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemProperties>
            for &'a IStorageItemPropertiesWithProvider
        {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemProperties> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItemProperties>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStorageItemPropertiesWithProvider_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IStorageFilePropertiesWithAvailability(::windows::Object);
        unsafe impl ::windows::Interface for IStorageFilePropertiesWithAvailability {
            type Vtable = IStorageFilePropertiesWithAvailability_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                2949365403,
                22571,
                16691,
                [150, 72, 228, 76, 164, 110, 228, 145],
            );
        }
        impl IStorageFilePropertiesWithAvailability {
            pub fn IsAvailable(&self) -> ::windows::Result<bool> {
                let this = self;
                unsafe {
                    let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<bool>(result__)
                }
            }
        }
        unsafe impl ::windows::RuntimeType for IStorageFilePropertiesWithAvailability {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{afcbbe9b-582b-4133-9648-e44ca46ee491}");
        }
        impl ::std::convert::From<IStorageFilePropertiesWithAvailability> for ::windows::Object {
            fn from(value: IStorageFilePropertiesWithAvailability) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IStorageFilePropertiesWithAvailability> for ::windows::Object {
            fn from(value: &IStorageFilePropertiesWithAvailability) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IStorageFilePropertiesWithAvailability {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object>
            for &'a IStorageFilePropertiesWithAvailability
        {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStorageFilePropertiesWithAvailability_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut bool,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IStorageFile2(::windows::Object);
        unsafe impl ::windows::Interface for IStorageFile2 {
            type Vtable = IStorageFile2_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                2504936399,
                2679,
                17147,
                [183, 119, 194, 237, 88, 165, 46, 68],
            );
        }
        impl IStorageFile2 {}
        unsafe impl ::windows::RuntimeType for IStorageFile2 {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{954e4bcf-0a77-42fb-b777-c2ed58a52e44}");
        }
        impl ::std::convert::From<IStorageFile2> for ::windows::Object {
            fn from(value: IStorageFile2) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IStorageFile2> for ::windows::Object {
            fn from(value: &IStorageFile2) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IStorageFile2 {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IStorageFile2 {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStorageFile2_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IStorageFileStatics2(::windows::Object);
        unsafe impl ::windows::Interface for IStorageFileStatics2 {
            type Vtable = IStorageFileStatics2_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                1551280001,
                8494,
                19193,
                [143, 4, 116, 12, 174, 16, 137, 116],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStorageFileStatics2_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IStorageFileStatics(::windows::Object);
        unsafe impl ::windows::Interface for IStorageFileStatics {
            type Vtable = IStorageFileStatics_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                1501873936,
                56050,
                17352,
                [139, 180, 164, 211, 234, 207, 208, 63],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStorageFileStatics_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                path: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
            pub unsafe extern "system" fn(),
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct StorageFile(::windows::Object);
        impl StorageFile {
            pub fn FileType(&self) -> ::windows::Result<::windows::HString> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn ContentType(&self) -> ::windows::Result<::windows::HString> {
                let this = self;
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn CopyOverloadDefaultNameAndOptions<'a>(
                &self,
                destinationfolder: impl ::windows::IntoParam<'a, IStorageFolder>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFile>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFile > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).10)(
                        ::windows::Abi::abi(this),
                        destinationfolder.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
                }
            }
            pub fn CopyOverloadDefaultOptions<'a>(
                &self,
                destinationfolder: impl ::windows::IntoParam<'a, IStorageFolder>,
                desirednewname: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFile>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFile > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        destinationfolder.into_param().abi(),
                        desirednewname.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
                }
            }
            pub fn IsAvailable(&self) -> ::windows::Result<bool> {
                let this =
                    &::windows::Interface::cast::<IStorageFilePropertiesWithAvailability>(self)
                        .unwrap();
                unsafe {
                    let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<bool>(result__)
                }
            }
            pub fn Name(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItem>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn Path(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItem>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).12)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn GetParentAsync(
                &self,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
                let this = &::windows::Interface::cast::<IStorageItem2>(self).unwrap();
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFolder > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
                }
            }
            pub fn IsEqual<'a>(
                &self,
                item: impl ::windows::IntoParam<'a, IStorageItem>,
            ) -> ::windows::Result<bool> {
                let this = &::windows::Interface::cast::<IStorageItem2>(self).unwrap();
                unsafe {
                    let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::windows::Abi::abi(this),
                        item.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<bool>(result__)
                }
            }
            pub fn DisplayName(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItemProperties>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).9)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn DisplayType(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItemProperties>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).10)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn FolderRelativeId(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItemProperties>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn GetFileFromPathAsync<'a>(
                path: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFile>> {
                Self::IStorageFileStatics(|this| unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFile > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).6)(
                        ::windows::Abi::abi(this),
                        path.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
                })
            }
            fn IStorageFileStatics<R, F: FnOnce(&IStorageFileStatics) -> ::windows::Result<R>>(
                callback: F,
            ) -> ::windows::Result<R> {
                static mut SHARED: ::windows::FactoryCache<StorageFile, IStorageFileStatics> =
                    ::windows::FactoryCache::new();
                unsafe { SHARED.call(callback) }
            }
            fn IStorageFileStatics2<R, F: FnOnce(&IStorageFileStatics2) -> ::windows::Result<R>>(
                callback: F,
            ) -> ::windows::Result<R> {
                static mut SHARED: ::windows::FactoryCache<StorageFile, IStorageFileStatics2> =
                    ::windows::FactoryCache::new();
                unsafe { SHARED.call(callback) }
            }
        }
        unsafe impl ::windows::RuntimeType for StorageFile {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                b"rc(Windows.Storage.StorageFile;{fa3f6186-4214-428c-a64c-14c9ac7315ea})",
            );
        }
        unsafe impl ::windows::Interface for StorageFile {
            type Vtable = IStorageFile_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                4198457734,
                16916,
                17036,
                [166, 76, 20, 201, 172, 115, 21, 234],
            );
        }
        impl ::windows::RuntimeName for StorageFile {
            const NAME: &'static str = "Windows.Storage.StorageFile";
        }
        impl ::std::convert::From<StorageFile> for ::windows::Object {
            fn from(value: StorageFile) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&StorageFile> for ::windows::Object {
            fn from(value: &StorageFile) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for StorageFile {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a StorageFile {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        impl ::std::convert::From<StorageFile> for IStorageFile {
            fn from(value: StorageFile) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl ::std::convert::From<&StorageFile> for IStorageFile {
            fn from(value: &StorageFile) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageFile> for StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageFile> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageFile>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageFile> for &'a StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageFile> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageFile>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<StorageFile> for IStorageFile2 {
            fn from(value: StorageFile) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFile> for IStorageFile2 {
            fn from(value: &StorageFile) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageFile2> for StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageFile2> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageFile2>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageFile2> for &'a StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageFile2> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageFile2>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<StorageFile> for IStorageFilePropertiesWithAvailability {
            fn from(value: StorageFile) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFile> for IStorageFilePropertiesWithAvailability {
            fn from(value: &StorageFile) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageFilePropertiesWithAvailability> for StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageFilePropertiesWithAvailability> {
                ::windows::Param::Owned(::std::convert::Into::<
                    IStorageFilePropertiesWithAvailability,
                >::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageFilePropertiesWithAvailability> for &'a StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageFilePropertiesWithAvailability> {
                ::windows::Param::Owned(::std::convert::Into::<
                    IStorageFilePropertiesWithAvailability,
                >::into(::std::clone::Clone::clone(self)))
            }
        }
        impl ::std::convert::From<StorageFile> for IStorageItem {
            fn from(value: StorageFile) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFile> for IStorageItem {
            fn from(value: &StorageFile) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItem> for StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageItem> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItem>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItem> for &'a StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageItem> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItem>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<StorageFile> for IStorageItem2 {
            fn from(value: StorageFile) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFile> for IStorageItem2 {
            fn from(value: &StorageFile) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItem2> for StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageItem2> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItem2>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItem2> for &'a StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageItem2> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItem2>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<StorageFile> for IStorageItemProperties {
            fn from(value: StorageFile) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFile> for IStorageItemProperties {
            fn from(value: &StorageFile) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemProperties> for StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemProperties> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItemProperties>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemProperties> for &'a StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemProperties> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItemProperties>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<StorageFile> for IStorageItemProperties2 {
            fn from(value: StorageFile) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFile> for IStorageItemProperties2 {
            fn from(value: &StorageFile) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemProperties2> for StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemProperties2> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItemProperties2>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemProperties2> for &'a StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemProperties2> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItemProperties2>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<StorageFile> for IStorageItemPropertiesWithProvider {
            fn from(value: StorageFile) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFile> for IStorageItemPropertiesWithProvider {
            fn from(value: &StorageFile) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemPropertiesWithProvider> for StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemPropertiesWithProvider> {
                ::windows::Param::Owned(
                    ::std::convert::Into::<IStorageItemPropertiesWithProvider>::into(self),
                )
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemPropertiesWithProvider> for &'a StorageFile {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemPropertiesWithProvider> {
                ::windows::Param::Owned(
                    ::std::convert::Into::<IStorageItemPropertiesWithProvider>::into(
                        ::std::clone::Clone::clone(self),
                    ),
                )
            }
        }
        impl ::std::convert::From<StorageFile> for Streams::IInputStreamReference {
            fn from(value: StorageFile) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFile> for Streams::IInputStreamReference {
            fn from(value: &StorageFile) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, Streams::IInputStreamReference> for StorageFile {
            fn into_param(self) -> ::windows::Param<'a, Streams::IInputStreamReference> {
                ::windows::Param::Owned(
                    ::std::convert::Into::<Streams::IInputStreamReference>::into(self),
                )
            }
        }
        impl<'a> ::windows::IntoParam<'a, Streams::IInputStreamReference> for &'a StorageFile {
            fn into_param(self) -> ::windows::Param<'a, Streams::IInputStreamReference> {
                ::windows::Param::Owned(
                    ::std::convert::Into::<Streams::IInputStreamReference>::into(
                        ::std::clone::Clone::clone(self),
                    ),
                )
            }
        }
        impl ::std::convert::From<StorageFile> for Streams::IRandomAccessStreamReference {
            fn from(value: StorageFile) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFile> for Streams::IRandomAccessStreamReference {
            fn from(value: &StorageFile) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, Streams::IRandomAccessStreamReference> for StorageFile {
            fn into_param(self) -> ::windows::Param<'a, Streams::IRandomAccessStreamReference> {
                ::windows::Param::Owned(
                    ::std::convert::Into::<Streams::IRandomAccessStreamReference>::into(self),
                )
            }
        }
        impl<'a> ::windows::IntoParam<'a, Streams::IRandomAccessStreamReference> for &'a StorageFile {
            fn into_param(self) -> ::windows::Param<'a, Streams::IRandomAccessStreamReference> {
                ::windows::Param::Owned(
                    ::std::convert::Into::<Streams::IRandomAccessStreamReference>::into(
                        ::std::clone::Clone::clone(self),
                    ),
                )
            }
        }
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IStorageFolder(::windows::Object);
        unsafe impl ::windows::Interface for IStorageFolder {
            type Vtable = IStorageFolder_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                1926351736,
                46063,
                20341,
                [168, 11, 111, 217, 218, 226, 148, 75],
            );
        }
        impl IStorageFolder {
            pub fn CreateFileAsyncOverloadDefaultOptions<'a>(
                &self,
                desiredname: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFile>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFile > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).6)(
                        ::windows::Abi::abi(this),
                        desiredname.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
                }
            }
            pub fn CreateFolderAsyncOverloadDefaultOptions<'a>(
                &self,
                desiredname: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFolder > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).8)(
                        ::windows::Abi::abi(this),
                        desiredname.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
                }
            }
            pub fn GetFileAsync<'a>(
                &self,
                name: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFile>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFile > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).10)(
                        ::windows::Abi::abi(this),
                        name.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
                }
            }
            pub fn GetFolderAsync<'a>(
                &self,
                name: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFolder > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        name.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
                }
            }
            pub fn GetItemAsync<'a>(
                &self,
                name: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<IStorageItem>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < IStorageItem > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).12)(
                        ::windows::Abi::abi(this),
                        name.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<IStorageItem>>(result__)
                }
            }
            pub fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(
                &self,
            ) -> ::windows::Result<
                super::Foundation::IAsyncOperation<
                    super::Foundation::Collections::IVectorView<StorageFile>,
                >,
            > {
                let this = self;
                unsafe {
                    let mut result__: <super::Foundation::IAsyncOperation<
                        super::Foundation::Collections::IVectorView<StorageFile>,
                    > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).13)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<
                        super::Foundation::Collections::IVectorView<StorageFile>,
                    >>(result__)
                }
            }
            pub fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(
                &self,
            ) -> ::windows::Result<
                super::Foundation::IAsyncOperation<
                    super::Foundation::Collections::IVectorView<StorageFolder>,
                >,
            > {
                let this = self;
                unsafe {
                    let mut result__: <super::Foundation::IAsyncOperation<
                        super::Foundation::Collections::IVectorView<StorageFolder>,
                    > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).14)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<
                        super::Foundation::Collections::IVectorView<StorageFolder>,
                    >>(result__)
                }
            }
            pub fn GetItemsAsyncOverloadDefaultStartAndCount(
                &self,
            ) -> ::windows::Result<
                super::Foundation::IAsyncOperation<
                    super::Foundation::Collections::IVectorView<IStorageItem>,
                >,
            > {
                let this = self;
                unsafe {
                    let mut result__: <super::Foundation::IAsyncOperation<
                        super::Foundation::Collections::IVectorView<IStorageItem>,
                    > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).15)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<
                        super::Foundation::Collections::IVectorView<IStorageItem>,
                    >>(result__)
                }
            }
            pub fn Name(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItem>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn Path(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItem>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).12)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
        }
        unsafe impl ::windows::RuntimeType for IStorageFolder {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{72d1cb78-b3ef-4f75-a80b-6fd9dae2944b}");
        }
        impl ::std::convert::From<IStorageFolder> for ::windows::Object {
            fn from(value: IStorageFolder) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IStorageFolder> for ::windows::Object {
            fn from(value: &IStorageFolder) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IStorageFolder {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IStorageFolder {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        impl ::std::convert::From<IStorageFolder> for IStorageItem {
            fn from(value: IStorageFolder) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&IStorageFolder> for IStorageItem {
            fn from(value: &IStorageFolder) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItem> for IStorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageItem> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItem>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItem> for &'a IStorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageItem> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItem>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStorageFolder_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                desiredname: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                desiredname: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                name: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                name: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                name: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct IStorageFolder2(::windows::Object);
        unsafe impl ::windows::Interface for IStorageFolder2 {
            type Vtable = IStorageFolder2_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                3894929593,
                2265,
                19086,
                [160, 172, 254, 94, 211, 203, 187, 211],
            );
        }
        impl IStorageFolder2 {
            pub fn TryGetItemAsync<'a>(
                &self,
                name: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<IStorageItem>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < IStorageItem > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).6)(
                        ::windows::Abi::abi(this),
                        name.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<IStorageItem>>(result__)
                }
            }
        }
        unsafe impl ::windows::RuntimeType for IStorageFolder2 {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer =
                ::windows::ConstBuffer::from_slice(b"{e827e8b9-08d9-4a8e-a0ac-fe5ed3cbbbd3}");
        }
        impl ::std::convert::From<IStorageFolder2> for ::windows::Object {
            fn from(value: IStorageFolder2) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&IStorageFolder2> for ::windows::Object {
            fn from(value: &IStorageFolder2) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IStorageFolder2 {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IStorageFolder2 {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStorageFolder2_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                name: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IStorageFolder3(::windows::Object);
        unsafe impl ::windows::Interface for IStorageFolder3 {
            type Vtable = IStorageFolder3_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                2673965209,
                48609,
                16676,
                [174, 179, 176, 106, 217, 111, 152, 212],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStorageFolder3_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IStorageFolderStatics2(::windows::Object);
        unsafe impl ::windows::Interface for IStorageFolderStatics2 {
            type Vtable = IStorageFolderStatics2_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                3026546115,
                29138,
                18045,
                [139, 41, 55, 31, 15, 98, 191, 111],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStorageFolderStatics2_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(),
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        #[doc(hidden)]
        pub struct IStorageFolderStatics(::windows::Object);
        unsafe impl ::windows::Interface for IStorageFolderStatics {
            type Vtable = IStorageFolderStatics_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                150153215,
                34261,
                18617,
                [174, 233, 40, 81, 30, 51, 159, 159],
            );
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStorageFolderStatics_abi(
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                iid: &::windows::Guid,
                interface: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                count: *mut u32,
                values: *mut *mut ::windows::Guid,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                value: *mut i32,
            ) -> ::windows::HRESULT,
            pub  unsafe extern "system" fn(
                this: ::windows::RawPtr,
                path: ::windows::RawPtr,
                result__: *mut ::windows::RawPtr,
            ) -> ::windows::HRESULT,
        );
        #[repr(transparent)]
        #[derive(
            :: std :: cmp :: PartialEq,
            :: std :: cmp :: Eq,
            :: std :: clone :: Clone,
            :: std :: fmt :: Debug,
        )]
        pub struct StorageFolder(::windows::Object);
        impl StorageFolder {
            pub fn CreateFileAsyncOverloadDefaultOptions<'a>(
                &self,
                desiredname: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFile>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFile > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).6)(
                        ::windows::Abi::abi(this),
                        desiredname.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
                }
            }
            pub fn CreateFolderAsyncOverloadDefaultOptions<'a>(
                &self,
                desiredname: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFolder > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).8)(
                        ::windows::Abi::abi(this),
                        desiredname.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
                }
            }
            pub fn GetFileAsync<'a>(
                &self,
                name: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFile>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFile > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).10)(
                        ::windows::Abi::abi(this),
                        name.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<StorageFile>>(result__)
                }
            }
            pub fn GetFolderAsync<'a>(
                &self,
                name: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFolder > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        name.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
                }
            }
            pub fn GetItemAsync<'a>(
                &self,
                name: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<IStorageItem>> {
                let this = self;
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < IStorageItem > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).12)(
                        ::windows::Abi::abi(this),
                        name.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<IStorageItem>>(result__)
                }
            }
            pub fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(
                &self,
            ) -> ::windows::Result<
                super::Foundation::IAsyncOperation<
                    super::Foundation::Collections::IVectorView<StorageFile>,
                >,
            > {
                let this = self;
                unsafe {
                    let mut result__: <super::Foundation::IAsyncOperation<
                        super::Foundation::Collections::IVectorView<StorageFile>,
                    > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).13)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<
                        super::Foundation::Collections::IVectorView<StorageFile>,
                    >>(result__)
                }
            }
            pub fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(
                &self,
            ) -> ::windows::Result<
                super::Foundation::IAsyncOperation<
                    super::Foundation::Collections::IVectorView<StorageFolder>,
                >,
            > {
                let this = self;
                unsafe {
                    let mut result__: <super::Foundation::IAsyncOperation<
                        super::Foundation::Collections::IVectorView<StorageFolder>,
                    > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).14)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<
                        super::Foundation::Collections::IVectorView<StorageFolder>,
                    >>(result__)
                }
            }
            pub fn GetItemsAsyncOverloadDefaultStartAndCount(
                &self,
            ) -> ::windows::Result<
                super::Foundation::IAsyncOperation<
                    super::Foundation::Collections::IVectorView<IStorageItem>,
                >,
            > {
                let this = self;
                unsafe {
                    let mut result__: <super::Foundation::IAsyncOperation<
                        super::Foundation::Collections::IVectorView<IStorageItem>,
                    > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).15)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<
                        super::Foundation::Collections::IVectorView<IStorageItem>,
                    >>(result__)
                }
            }
            pub fn TryGetItemAsync<'a>(
                &self,
                name: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<IStorageItem>> {
                let this = &::windows::Interface::cast::<IStorageFolder2>(self).unwrap();
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < IStorageItem > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).6)(
                        ::windows::Abi::abi(this),
                        name.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<IStorageItem>>(result__)
                }
            }
            pub fn Name(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItem>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn Path(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItem>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).12)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn GetParentAsync(
                &self,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
                let this = &::windows::Interface::cast::<IStorageItem2>(self).unwrap();
                unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFolder > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).6)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
                }
            }
            pub fn IsEqual<'a>(
                &self,
                item: impl ::windows::IntoParam<'a, IStorageItem>,
            ) -> ::windows::Result<bool> {
                let this = &::windows::Interface::cast::<IStorageItem2>(self).unwrap();
                unsafe {
                    let mut result__: <bool as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).7)(
                        ::windows::Abi::abi(this),
                        item.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<bool>(result__)
                }
            }
            pub fn DisplayName(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItemProperties>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).9)(::windows::Abi::abi(this), &mut result__)
                        .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn DisplayType(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItemProperties>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).10)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn FolderRelativeId(&self) -> ::windows::Result<::windows::HString> {
                let this = &::windows::Interface::cast::<IStorageItemProperties>(self).unwrap();
                unsafe {
                    let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                        ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).11)(
                        ::windows::Abi::abi(this),
                        &mut result__,
                    )
                    .from_abi::<::windows::HString>(result__)
                }
            }
            pub fn GetItemsAsync(
                &self,
                startindex: u32,
                maxitemstoretrieve: u32,
            ) -> ::windows::Result<
                super::Foundation::IAsyncOperation<
                    super::Foundation::Collections::IVectorView<IStorageItem>,
                >,
            > {
                let this =
                    &::windows::Interface::cast::<Search::IStorageFolderQueryOperations>(self)
                        .unwrap();
                unsafe {
                    let mut result__: <super::Foundation::IAsyncOperation<
                        super::Foundation::Collections::IVectorView<IStorageItem>,
                    > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                    (::windows::Interface::vtable(this).19)(
                        ::windows::Abi::abi(this),
                        startindex,
                        maxitemstoretrieve,
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<
                        super::Foundation::Collections::IVectorView<IStorageItem>,
                    >>(result__)
                }
            }
            pub fn GetFolderFromPathAsync<'a>(
                path: impl ::windows::IntoParam<'a, ::windows::HString>,
            ) -> ::windows::Result<super::Foundation::IAsyncOperation<StorageFolder>> {
                Self::IStorageFolderStatics(|this| unsafe {
                    let mut result__ : < super :: Foundation :: IAsyncOperation :: < StorageFolder > as :: windows :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
                    (::windows::Interface::vtable(this).6)(
                        ::windows::Abi::abi(this),
                        path.into_param().abi(),
                        &mut result__,
                    )
                    .from_abi::<super::Foundation::IAsyncOperation<StorageFolder>>(result__)
                })
            }
            fn IStorageFolderStatics<
                R,
                F: FnOnce(&IStorageFolderStatics) -> ::windows::Result<R>,
            >(
                callback: F,
            ) -> ::windows::Result<R> {
                static mut SHARED: ::windows::FactoryCache<StorageFolder, IStorageFolderStatics> =
                    ::windows::FactoryCache::new();
                unsafe { SHARED.call(callback) }
            }
            fn IStorageFolderStatics2<
                R,
                F: FnOnce(&IStorageFolderStatics2) -> ::windows::Result<R>,
            >(
                callback: F,
            ) -> ::windows::Result<R> {
                static mut SHARED: ::windows::FactoryCache<StorageFolder, IStorageFolderStatics2> =
                    ::windows::FactoryCache::new();
                unsafe { SHARED.call(callback) }
            }
        }
        unsafe impl ::windows::RuntimeType for StorageFolder {
            type DefaultType = ::std::option::Option<Self>;
            const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                b"rc(Windows.Storage.StorageFolder;{72d1cb78-b3ef-4f75-a80b-6fd9dae2944b})",
            );
        }
        unsafe impl ::windows::Interface for StorageFolder {
            type Vtable = IStorageFolder_abi;
            const IID: ::windows::Guid = ::windows::Guid::from_values(
                1926351736,
                46063,
                20341,
                [168, 11, 111, 217, 218, 226, 148, 75],
            );
        }
        impl ::windows::RuntimeName for StorageFolder {
            const NAME: &'static str = "Windows.Storage.StorageFolder";
        }
        impl ::std::convert::From<StorageFolder> for ::windows::Object {
            fn from(value: StorageFolder) -> Self {
                value.0
            }
        }
        impl ::std::convert::From<&StorageFolder> for ::windows::Object {
            fn from(value: &StorageFolder) -> Self {
                value.0.clone()
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Owned(self.0)
            }
        }
        impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                ::windows::Param::Borrowed(&self.0)
            }
        }
        impl ::std::convert::From<StorageFolder> for IStorageFolder {
            fn from(value: StorageFolder) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl ::std::convert::From<&StorageFolder> for IStorageFolder {
            fn from(value: &StorageFolder) -> Self {
                ::std::convert::From::from(::std::clone::Clone::clone(value))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageFolder> for StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageFolder> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageFolder>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageFolder> for &'a StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageFolder> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageFolder>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<StorageFolder> for IStorageFolder2 {
            fn from(value: StorageFolder) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFolder> for IStorageFolder2 {
            fn from(value: &StorageFolder) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageFolder2> for StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageFolder2> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageFolder2>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageFolder2> for &'a StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageFolder2> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageFolder2>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<StorageFolder> for IStorageItem {
            fn from(value: StorageFolder) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFolder> for IStorageItem {
            fn from(value: &StorageFolder) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItem> for StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageItem> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItem>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItem> for &'a StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageItem> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItem>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<StorageFolder> for IStorageItem2 {
            fn from(value: StorageFolder) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFolder> for IStorageItem2 {
            fn from(value: &StorageFolder) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItem2> for StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageItem2> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItem2>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItem2> for &'a StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageItem2> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItem2>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<StorageFolder> for IStorageItemProperties {
            fn from(value: StorageFolder) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFolder> for IStorageItemProperties {
            fn from(value: &StorageFolder) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemProperties> for StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemProperties> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItemProperties>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemProperties> for &'a StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemProperties> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItemProperties>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<StorageFolder> for IStorageItemProperties2 {
            fn from(value: StorageFolder) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFolder> for IStorageItemProperties2 {
            fn from(value: &StorageFolder) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemProperties2> for StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemProperties2> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItemProperties2>::into(self))
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemProperties2> for &'a StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemProperties2> {
                ::windows::Param::Owned(::std::convert::Into::<IStorageItemProperties2>::into(
                    ::std::clone::Clone::clone(self),
                ))
            }
        }
        impl ::std::convert::From<StorageFolder> for IStorageItemPropertiesWithProvider {
            fn from(value: StorageFolder) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFolder> for IStorageItemPropertiesWithProvider {
            fn from(value: &StorageFolder) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemPropertiesWithProvider> for StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemPropertiesWithProvider> {
                ::windows::Param::Owned(
                    ::std::convert::Into::<IStorageItemPropertiesWithProvider>::into(self),
                )
            }
        }
        impl<'a> ::windows::IntoParam<'a, IStorageItemPropertiesWithProvider> for &'a StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, IStorageItemPropertiesWithProvider> {
                ::windows::Param::Owned(
                    ::std::convert::Into::<IStorageItemPropertiesWithProvider>::into(
                        ::std::clone::Clone::clone(self),
                    ),
                )
            }
        }
        impl ::std::convert::From<StorageFolder> for Search::IStorageFolderQueryOperations {
            fn from(value: StorageFolder) -> Self {
                ::std::convert::From::from(&value)
            }
        }
        impl ::std::convert::From<&StorageFolder> for Search::IStorageFolderQueryOperations {
            fn from(value: &StorageFolder) -> Self {
                ::windows::Interface::cast(value).unwrap()
            }
        }
        impl<'a> ::windows::IntoParam<'a, Search::IStorageFolderQueryOperations> for StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, Search::IStorageFolderQueryOperations> {
                ::windows::Param::Owned(
                    ::std::convert::Into::<Search::IStorageFolderQueryOperations>::into(self),
                )
            }
        }
        impl<'a> ::windows::IntoParam<'a, Search::IStorageFolderQueryOperations> for &'a StorageFolder {
            fn into_param(self) -> ::windows::Param<'a, Search::IStorageFolderQueryOperations> {
                ::windows::Param::Owned(
                    ::std::convert::Into::<Search::IStorageFolderQueryOperations>::into(
                        ::std::clone::Clone::clone(self),
                    ),
                )
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Pickers {
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IFileOpenPicker2(::windows::Object);
            unsafe impl ::windows::Interface for IFileOpenPicker2 {
                type Vtable = IFileOpenPicker2_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    2364239058,
                    46150,
                    18167,
                    [178, 101, 144, 248, 229, 90, 214, 80],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IFileOpenPicker2_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IFileOpenPickerWithOperationId(::windows::Object);
            unsafe impl ::windows::Interface for IFileOpenPickerWithOperationId {
                type Vtable = IFileOpenPickerWithOperationId_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    1062712681,
                    9506,
                    19621,
                    [170, 115, 161, 85, 9, 241, 252, 191],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IFileOpenPickerWithOperationId_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    pickeroperationid: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IFileOpenPicker(::windows::Object);
            unsafe impl ::windows::Interface for IFileOpenPicker {
                type Vtable = IFileOpenPicker_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    749217674,
                    4805,
                    19551,
                    [137, 119, 148, 84, 119, 147, 194, 65],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IFileOpenPicker_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut PickerViewMode,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: PickerViewMode,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut PickerLocationId,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: PickerLocationId,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IFileOpenPicker3(::windows::Object);
            unsafe impl ::windows::Interface for IFileOpenPicker3 {
                type Vtable = IFileOpenPicker3_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    3651519923,
                    50652,
                    23448,
                    [189, 128, 168, 208, 202, 5, 132, 216],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IFileOpenPicker3_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IFileOpenPickerStatics(::windows::Object);
            unsafe impl ::windows::Interface for IFileOpenPickerStatics {
                type Vtable = IFileOpenPickerStatics_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    1747015483,
                    12034,
                    18483,
                    [150, 212, 171, 191, 173, 114, 182, 123],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IFileOpenPickerStatics_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            #[doc(hidden)]
            pub struct IFileOpenPickerStatics2(::windows::Object);
            unsafe impl ::windows::Interface for IFileOpenPickerStatics2 {
                type Vtable = IFileOpenPickerStatics2_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    3901846549,
                    60893,
                    23704,
                    [182, 243, 54, 111, 223, 202, 211, 146],
                );
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IFileOpenPickerStatics2_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct FileOpenPicker(::windows::Object);
            impl FileOpenPicker {
                pub fn new() -> ::windows::Result<Self> {
                    Self::IActivationFactory(|f| f.activate_instance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(&::windows::IActivationFactory) -> ::windows::Result<R>,
                >(
                    callback: F,
                ) -> ::windows::Result<R> {
                    static mut SHARED: ::windows::FactoryCache<
                        FileOpenPicker,
                        ::windows::IActivationFactory,
                    > = ::windows::FactoryCache::new();
                    unsafe { SHARED.call(callback) }
                }
                pub fn ViewMode(&self) -> ::windows::Result<PickerViewMode> {
                    let this = self;
                    unsafe {
                        let mut result__: <PickerViewMode as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<PickerViewMode>(result__)
                    }
                }
                pub fn SetViewMode(&self, value: PickerViewMode) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn SettingsIdentifier(&self) -> ::windows::Result<::windows::HString> {
                    let this = self;
                    unsafe {
                        let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).8)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HString>(result__)
                    }
                }
                pub fn SetSettingsIdentifier<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HString>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).9)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn SuggestedStartLocation(&self) -> ::windows::Result<PickerLocationId> {
                    let this = self;
                    unsafe {
                        let mut result__: <PickerLocationId as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).10)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<PickerLocationId>(result__)
                    }
                }
                pub fn SetSuggestedStartLocation(
                    &self,
                    value: PickerLocationId,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).11)(::windows::Abi::abi(this), value)
                            .ok()
                    }
                }
                pub fn CommitButtonText(&self) -> ::windows::Result<::windows::HString> {
                    let this = self;
                    unsafe {
                        let mut result__: <::windows::HString as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).12)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<::windows::HString>(result__)
                    }
                }
                pub fn SetCommitButtonText<'a>(
                    &self,
                    value: impl ::windows::IntoParam<'a, ::windows::HString>,
                ) -> ::windows::Result<()> {
                    let this = self;
                    unsafe {
                        (::windows::Interface::vtable(this).13)(
                            ::windows::Abi::abi(this),
                            value.into_param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn FileTypeFilter(
                    &self,
                ) -> ::windows::Result<
                    super::super::Foundation::Collections::IVector<::windows::HString>,
                > {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::Collections::IVector<
                            ::windows::HString,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        ( :: windows :: Interface :: vtable ( this ) .14 ) ( :: windows :: Abi :: abi ( this ) , & mut result__ ) . from_abi :: < super :: super :: Foundation :: Collections :: IVector :: < :: windows :: HString > > ( result__ )
                    }
                }
                pub fn PickSingleFileAsync(
                    &self,
                ) -> ::windows::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>
                {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::IAsyncOperation<
                            super::StorageFile,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).15)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(
                            result__,
                        )
                    }
                }
                pub fn PickMultipleFilesAsync(
                    &self,
                ) -> ::windows::Result<
                    super::super::Foundation::IAsyncOperation<
                        super::super::Foundation::Collections::IVectorView<super::StorageFile>,
                    >,
                > {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::IAsyncOperation<
                            super::super::Foundation::Collections::IVectorView<super::StorageFile>,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).16)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::IAsyncOperation<
                            super::super::Foundation::Collections::IVectorView<super::StorageFile>,
                        >>(result__)
                    }
                }
                #[cfg(feature = "deprecated")]
                pub fn PickSingleFileAndContinue(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IFileOpenPicker2>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).7)(::windows::Abi::abi(this)).ok()
                    }
                }
                #[cfg(feature = "deprecated")]
                pub fn PickMultipleFilesAndContinue(&self) -> ::windows::Result<()> {
                    let this = &::windows::Interface::cast::<IFileOpenPicker2>(self).unwrap();
                    unsafe {
                        (::windows::Interface::vtable(this).8)(::windows::Abi::abi(this)).ok()
                    }
                }
                pub fn PickSingleFileAsync2<'a>(
                    &self,
                    pickeroperationid: impl ::windows::IntoParam<'a, ::windows::HString>,
                ) -> ::windows::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>
                {
                    let this = &::windows::Interface::cast::<IFileOpenPickerWithOperationId>(self)
                        .unwrap();
                    unsafe {
                        let mut result__: <super::super::Foundation::IAsyncOperation<
                            super::StorageFile,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            pickeroperationid.into_param().abi(),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(
                            result__,
                        )
                    }
                }
                #[cfg(feature = "deprecated")]
                pub fn ResumePickSingleFileAsync(
                ) -> ::windows::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>
                {
                    Self::IFileOpenPickerStatics(|this| unsafe {
                        let mut result__: <super::super::Foundation::IAsyncOperation<
                            super::StorageFile,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).6)(
                            ::windows::Abi::abi(this),
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(
                            result__,
                        )
                    })
                }
                fn IFileOpenPickerStatics<
                    R,
                    F: FnOnce(&IFileOpenPickerStatics) -> ::windows::Result<R>,
                >(
                    callback: F,
                ) -> ::windows::Result<R> {
                    static mut SHARED: ::windows::FactoryCache<
                        FileOpenPicker,
                        IFileOpenPickerStatics,
                    > = ::windows::FactoryCache::new();
                    unsafe { SHARED.call(callback) }
                }
                fn IFileOpenPickerStatics2<
                    R,
                    F: FnOnce(&IFileOpenPickerStatics2) -> ::windows::Result<R>,
                >(
                    callback: F,
                ) -> ::windows::Result<R> {
                    static mut SHARED: ::windows::FactoryCache<
                        FileOpenPicker,
                        IFileOpenPickerStatics2,
                    > = ::windows::FactoryCache::new();
                    unsafe { SHARED.call(callback) }
                }
            }
            unsafe impl ::windows::RuntimeType for FileOpenPicker {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE : :: windows :: ConstBuffer = :: windows :: ConstBuffer :: from_slice ( b"rc(Windows.Storage.Pickers.FileOpenPicker;{2ca8278a-12c5-4c5f-8977-94547793c241})" ) ;
            }
            unsafe impl ::windows::Interface for FileOpenPicker {
                type Vtable = IFileOpenPicker_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    749217674,
                    4805,
                    19551,
                    [137, 119, 148, 84, 119, 147, 194, 65],
                );
            }
            impl ::windows::RuntimeName for FileOpenPicker {
                const NAME: &'static str = "Windows.Storage.Pickers.FileOpenPicker";
            }
            impl ::std::convert::From<FileOpenPicker> for ::windows::Object {
                fn from(value: FileOpenPicker) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&FileOpenPicker> for ::windows::Object {
                fn from(value: &FileOpenPicker) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for FileOpenPicker {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a FileOpenPicker {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            unsafe impl ::std::marker::Send for FileOpenPicker {}
            unsafe impl ::std::marker::Sync for FileOpenPicker {}
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: marker :: Copy,
                :: std :: clone :: Clone,
                :: std :: default :: Default,
                :: std :: fmt :: Debug,
            )]
            #[repr(transparent)]
            pub struct PickerLocationId(pub i32);
            impl PickerLocationId {
                pub const DocumentsLibrary: Self = Self(0i32);
                pub const ComputerFolder: Self = Self(1i32);
                pub const Desktop: Self = Self(2i32);
                pub const Downloads: Self = Self(3i32);
                pub const HomeGroup: Self = Self(4i32);
                pub const MusicLibrary: Self = Self(5i32);
                pub const PicturesLibrary: Self = Self(6i32);
                pub const VideosLibrary: Self = Self(7i32);
                pub const Objects3D: Self = Self(8i32);
                pub const Unspecified: Self = Self(9i32);
            }
            impl ::std::convert::From<i32> for PickerLocationId {
                fn from(value: i32) -> Self {
                    Self(value)
                }
            }
            unsafe impl ::windows::Abi for PickerLocationId {
                type Abi = Self;
            }
            unsafe impl ::windows::RuntimeType for PickerLocationId {
                type DefaultType = Self;
                const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                    b"enum(Windows.Storage.Pickers.PickerLocationId;i4)",
                );
            }
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: marker :: Copy,
                :: std :: clone :: Clone,
                :: std :: default :: Default,
                :: std :: fmt :: Debug,
            )]
            #[repr(transparent)]
            pub struct PickerViewMode(pub i32);
            impl PickerViewMode {
                pub const List: Self = Self(0i32);
                pub const Thumbnail: Self = Self(1i32);
            }
            impl ::std::convert::From<i32> for PickerViewMode {
                fn from(value: i32) -> Self {
                    Self(value)
                }
            }
            unsafe impl ::windows::Abi for PickerViewMode {
                type Abi = Self;
            }
            unsafe impl ::windows::RuntimeType for PickerViewMode {
                type DefaultType = Self;
                const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                    b"enum(Windows.Storage.Pickers.PickerViewMode;i4)",
                );
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Search {
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IStorageFolderQueryOperations(::windows::Object);
            unsafe impl ::windows::Interface for IStorageFolderQueryOperations {
                type Vtable = IStorageFolderQueryOperations_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    3410218185,
                    17515,
                    19023,
                    [190, 151, 117, 119, 113, 190, 82, 3],
                );
            }
            impl IStorageFolderQueryOperations {
                pub fn GetItemsAsync(
                    &self,
                    startindex: u32,
                    maxitemstoretrieve: u32,
                ) -> ::windows::Result<
                    super::super::Foundation::IAsyncOperation<
                        super::super::Foundation::Collections::IVectorView<super::IStorageItem>,
                    >,
                > {
                    let this = self;
                    unsafe {
                        let mut result__: <super::super::Foundation::IAsyncOperation<
                            super::super::Foundation::Collections::IVectorView<super::IStorageItem>,
                        > as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(this).19)(
                            ::windows::Abi::abi(this),
                            startindex,
                            maxitemstoretrieve,
                            &mut result__,
                        )
                        .from_abi::<super::super::Foundation::IAsyncOperation<
                            super::super::Foundation::Collections::IVectorView<super::IStorageItem>,
                        >>(result__)
                    }
                }
            }
            unsafe impl ::windows::RuntimeType for IStorageFolderQueryOperations {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer =
                    ::windows::ConstBuffer::from_slice(b"{cb43ccc9-446b-4a4f-be97-757771be5203}");
            }
            impl ::std::convert::From<IStorageFolderQueryOperations> for ::windows::Object {
                fn from(value: IStorageFolderQueryOperations) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&IStorageFolderQueryOperations> for ::windows::Object {
                fn from(value: &IStorageFolderQueryOperations) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IStorageFolderQueryOperations {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IStorageFolderQueryOperations {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IStorageFolderQueryOperations_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    startindex: u32,
                    maxitemstoretrieve: u32,
                    result__: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
                pub unsafe extern "system" fn(),
            );
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Streams {
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IRandomAccessStreamReference(::windows::Object);
            unsafe impl ::windows::Interface for IRandomAccessStreamReference {
                type Vtable = IRandomAccessStreamReference_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    871248180,
                    7638,
                    20026,
                    [128, 103, 209, 193, 98, 232, 100, 43],
                );
            }
            impl IRandomAccessStreamReference {}
            unsafe impl ::windows::RuntimeType for IRandomAccessStreamReference {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer =
                    ::windows::ConstBuffer::from_slice(b"{33ee3134-1dd6-4e3a-8067-d1c162e8642b}");
            }
            impl ::std::convert::From<IRandomAccessStreamReference> for ::windows::Object {
                fn from(value: IRandomAccessStreamReference) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&IRandomAccessStreamReference> for ::windows::Object {
                fn from(value: &IRandomAccessStreamReference) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IRandomAccessStreamReference {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IRandomAccessStreamReference {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IRandomAccessStreamReference_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
            );
            #[repr(transparent)]
            #[derive(
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: clone :: Clone,
                :: std :: fmt :: Debug,
            )]
            pub struct IInputStreamReference(::windows::Object);
            unsafe impl ::windows::Interface for IInputStreamReference {
                type Vtable = IInputStreamReference_abi;
                const IID: ::windows::Guid = ::windows::Guid::from_values(
                    1133681944,
                    24265,
                    19290,
                    [145, 156, 66, 5, 176, 200, 4, 182],
                );
            }
            impl IInputStreamReference {}
            unsafe impl ::windows::RuntimeType for IInputStreamReference {
                type DefaultType = ::std::option::Option<Self>;
                const SIGNATURE: ::windows::ConstBuffer =
                    ::windows::ConstBuffer::from_slice(b"{43929d18-5ec9-4b5a-919c-4205b0c804b6}");
            }
            impl ::std::convert::From<IInputStreamReference> for ::windows::Object {
                fn from(value: IInputStreamReference) -> Self {
                    value.0
                }
            }
            impl ::std::convert::From<&IInputStreamReference> for ::windows::Object {
                fn from(value: &IInputStreamReference) -> Self {
                    value.0.clone()
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for IInputStreamReference {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Owned(self.0)
                }
            }
            impl<'a> ::windows::IntoParam<'a, ::windows::Object> for &'a IInputStreamReference {
                fn into_param(self) -> ::windows::Param<'a, ::windows::Object> {
                    ::windows::Param::Borrowed(&self.0)
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IInputStreamReference_abi(
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    iid: &::windows::Guid,
                    interface: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    count: *mut u32,
                    values: *mut *mut ::windows::Guid,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut ::windows::RawPtr,
                ) -> ::windows::HRESULT,
                pub  unsafe extern "system" fn(
                    this: ::windows::RawPtr,
                    value: *mut i32,
                ) -> ::windows::HRESULT,
                pub unsafe extern "system" fn(),
            );
        }
    }
}
