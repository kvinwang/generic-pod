#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
extern crate alloc;
pub use ticket_office::{Extracted, Recharged, TicketOffice};
mod ticket_office {
    impl ::ink::env::ContractEnv for TicketOffice {
        type Env = PinkEnvironment;
    }
    type Environment = <TicketOffice as ::ink::env::ContractEnv>::Env;
    type AccountId =
        <<TicketOffice as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::AccountId;
    type Balance =
        <<TicketOffice as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Balance;
    type Hash = <<TicketOffice as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Hash;
    type Timestamp =
        <<TicketOffice as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Timestamp;
    type BlockNumber =
        <<TicketOffice as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::BlockNumber;
    type ChainExtension =
        <<TicketOffice as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::ChainExtension;
    const MAX_EVENT_TOPICS : usize = < < TicketOffice as :: ink :: env :: ContractEnv > :: Env as :: ink :: env :: Environment > :: MAX_EVENT_TOPICS ;
    const _: () = {
        struct Check {
            salt: (),
            field_0: AccountId,
        }
    };
    #[cfg(not(feature = "__ink_dylint_Storage"))]
    pub struct TicketOffice {
        #[doc = " Owner of the contract"]
        owner: <AccountId as ::ink::storage::traits::AutoStorableHint<
            ::ink::storage::traits::ManualKey<2788005635u32, ()>,
        >>::Type,
    }
    const _: () = {
        impl<__ink_generic_salt: ::ink::storage::traits::StorageKey>
            ::ink::storage::traits::StorableHint<__ink_generic_salt> for TicketOffice
        {
            type Type = TicketOffice;
            type PreferredKey = ::ink::storage::traits::AutoKey;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageKey for TicketOffice {
            const KEY: ::ink::primitives::Key = <() as ::ink::storage::traits::StorageKey>::KEY;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::Storable for TicketOffice {
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn decode<__ink_I: ::scale::Input>(
                __input: &mut __ink_I,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(TicketOffice {
                    owner: <<AccountId as ::ink::storage::traits::AutoStorableHint<
                        ::ink::storage::traits::ManualKey<2788005635u32, ()>,
                    >>::Type as ::ink::storage::traits::Storable>::decode(
                        __input
                    )?,
                })
            }
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn encode<__ink_O: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __dest: &mut __ink_O,
            ) {
                match self {
                    TicketOffice { owner: __binding_0 } => {
                        ::ink::storage::traits::Storable::encode(__binding_0, __dest);
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for TicketOffice {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                :: scale_info :: Type :: builder () . path (:: scale_info :: Path :: new_with_replace ("TicketOffice" , "pod_ticket_house::ticket_office" , & [])) . type_params (:: alloc :: vec :: Vec :: new ()) . composite (:: scale_info :: build :: Fields :: named () . field (| f | f . ty :: < < AccountId as :: ink :: storage :: traits :: AutoStorableHint < :: ink :: storage :: traits :: ManualKey < 2788005635u32 , () > > > :: Type > () . name ("owner") . type_name ("<AccountId as::ink::storage::traits::AutoStorableHint<::ink::\nstorage::traits::ManualKey<2788005635u32, ()>,>>::Type") . docs (& ["Owner of the contract"])))
            }
        };
    };
    const _: () = {
        impl ::ink::storage::traits::StorageLayout for TicketOffice {
            fn layout(__key: &::ink::primitives::Key) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(::ink::metadata::layout::StructLayout::new(
                    "TicketOffice",
                    [::ink::metadata::layout::FieldLayout::new(
                        "owner",
                        <<AccountId as ::ink::storage::traits::AutoStorableHint<
                            ::ink::storage::traits::ManualKey<2788005635u32, ()>,
                        >>::Type as ::ink::storage::traits::StorageLayout>::layout(
                            __key
                        ),
                    )],
                ))
            }
        }
    };
    const _: () = {
        impl ::ink::reflect::ContractName for TicketOffice {
            const NAME: &'static str = "TicketOffice";
        }
    };
    const _: () = {
        impl<'a> ::ink::codegen::Env for &'a TicketOffice {
            type EnvAccess = ::ink::EnvAccess<'a, <TicketOffice as ::ink::env::ContractEnv>::Env>;
            fn env(self) -> Self::EnvAccess {
                <<Self as ::ink::codegen::Env>::EnvAccess as ::core::default::Default>::default()
            }
        }
        impl<'a> ::ink::codegen::StaticEnv for TicketOffice {
            type EnvAccess =
                ::ink::EnvAccess<'static, <TicketOffice as ::ink::env::ContractEnv>::Env>;
            fn env() -> Self::EnvAccess {
                < < Self as :: ink :: codegen :: StaticEnv > :: EnvAccess as :: core :: default :: Default > :: default ()
            }
        }
    };
    const _: () = {
        #[allow(unused_imports)]
        use ::ink::codegen::{Env as _, StaticEnv as _};
        use ::ink::codegen::EmitEvent as _;
    };
    const _: () = {
        impl<'a> ::ink::codegen::EmitEvent<TicketOffice> for ::ink::EnvAccess<'a, Environment> {
            fn emit_event<E>(self, event: E)
            where
                E: Into<<TicketOffice as ::ink::reflect::ContractEventBase>::Type>,
            {
                ::ink::env::emit_event::<
                    Environment,
                    <TicketOffice as ::ink::reflect::ContractEventBase>::Type,
                >(event.into());
            }
        }
    };
    #[allow(non_camel_case_types)]
    #[cfg(not(feature = "__ink_dylint_EventBase"))]
    pub enum __ink_EventBase {
        Extracted(Extracted),
        Recharged(Recharged),
    }
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl ::scale::Encode for __ink_EventBase {
            fn size_hint(&self) -> usize {
                1_usize
                    + match *self {
                        __ink_EventBase::Extracted(ref aa) => {
                            0_usize.saturating_add(::scale::Encode::size_hint(aa))
                        }
                        __ink_EventBase::Recharged(ref aa) => {
                            0_usize.saturating_add(::scale::Encode::size_hint(aa))
                        }
                        _ => 0_usize,
                    }
            }
            fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    __ink_EventBase::Extracted(ref aa) => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        ::scale::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    __ink_EventBase::Recharged(ref aa) => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                        ::scale::Encode::encode_to(aa, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        #[automatically_derived]
        impl ::scale::EncodeLike for __ink_EventBase {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl ::scale::Decode for __ink_EventBase {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                match __codec_input_edqy.read_byte().map_err(|e| {
                    e.chain("Could not decode `__ink_EventBase`, failed to read variant byte")
                })? {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Ok(__ink_EventBase::Extracted({
                                let __codec_res_edqy =
                                    <Extracted as ::scale::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(e.chain(
                                            "Could not decode `__ink_EventBase::Extracted.0`",
                                        ))
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }))
                        })();
                    }
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Ok(__ink_EventBase::Recharged({
                                let __codec_res_edqy =
                                    <Recharged as ::scale::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(e.chain(
                                            "Could not decode `__ink_EventBase::Recharged.0`",
                                        ))
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }))
                        })();
                    }
                    _ => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                                "Could not decode `__ink_EventBase`, variant doesn't exist",
                            ))
                        })();
                    }
                }
            }
        }
    };
    const _: () = {
        impl ::ink::reflect::ContractEventBase for TicketOffice {
            type Type = __ink_EventBase;
        }
    };
    const _: () = {
        impl From<Extracted> for __ink_EventBase {
            fn from(event: Extracted) -> Self {
                Self::Extracted(event)
            }
        }
    };
    const _: () = {
        impl From<Recharged> for __ink_EventBase {
            fn from(event: Recharged) -> Self {
                Self::Recharged(event)
            }
        }
    };
    const _: () = {
        pub enum __ink_UndefinedAmountOfTopics {}
        impl ::ink::env::topics::EventTopicsAmount for __ink_UndefinedAmountOfTopics {
            const AMOUNT: usize = 0;
        }
        impl ::ink::env::Topics for __ink_EventBase {
            type RemainingTopics = __ink_UndefinedAmountOfTopics;
            fn topics<E, B>(
                &self,
                builder: ::ink::env::topics::TopicsBuilder<::ink::env::topics::state::Uninit, E, B>,
            ) -> <B as ::ink::env::topics::TopicsBuilderBackend<E>>::Output
            where
                E: ::ink::env::Environment,
                B: ::ink::env::topics::TopicsBuilderBackend<E>,
            {
                match self {
                    Self::Extracted(event) => {
                        <Extracted as ::ink::env::Topics>::topics::<E, B>(event, builder)
                    }
                    Self::Recharged(event) => {
                        <Recharged as ::ink::env::Topics>::topics::<E, B>(event, builder)
                    }
                    _ => {
                        ::core::panicking::panic_fmt(format_args!("Event does not exist!"));
                    }
                }
            }
        }
    };
    impl ::ink::codegen::EventLenTopics for Extracted {
        type LenTopics = ::ink::codegen::EventTopics<0usize>;
    }
    const _: () = ::ink::codegen::utils::consume_type::<
        ::ink::codegen::EventRespectsTopicLimit<
            Extracted,
            {
                < < TicketOffice as :: ink :: env :: ContractEnv > :: Env as :: ink :: env :: Environment > :: MAX_EVENT_TOPICS
            },
        >,
    >();
    impl ::ink::codegen::EventLenTopics for Recharged {
        type LenTopics = ::ink::codegen::EventTopics<0usize>;
    }
    const _: () = ::ink::codegen::utils::consume_type::<
        ::ink::codegen::EventRespectsTopicLimit<
            Recharged,
            {
                < < TicketOffice as :: ink :: env :: ContractEnv > :: Env as :: ink :: env :: Environment > :: MAX_EVENT_TOPICS
            },
        >,
    >();
    pub struct Extracted {
        dest: AccountId,
        amount: Balance,
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Encode for Extracted {
            fn size_hint(&self) -> usize {
                0_usize
                    .saturating_add(::scale::Encode::size_hint(&self.dest))
                    .saturating_add(::scale::Encode::size_hint(&self.amount))
            }
            fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::scale::Encode::encode_to(&self.dest, __codec_dest_edqy);
                ::scale::Encode::encode_to(&self.amount, __codec_dest_edqy);
            }
        }
        #[automatically_derived]
        impl ::scale::EncodeLike for Extracted {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Decode for Extracted {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(Extracted {
                    dest: {
                        let __codec_res_edqy =
                            <AccountId as ::scale::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Extracted::dest`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    amount: {
                        let __codec_res_edqy =
                            <Balance as ::scale::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Extracted::amount`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                })
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Extracted {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Extracted",
                "dest",
                &self.dest,
                "amount",
                &&self.amount,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Extracted {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Extracted {
        #[inline]
        fn eq(&self, other: &Extracted) -> bool {
            self.dest == other.dest && self.amount == other.amount
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Extracted {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Extracted {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Balance>;
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Extracted {
        #[inline]
        fn clone(&self) -> Extracted {
            Extracted {
                dest: ::core::clone::Clone::clone(&self.dest),
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for Extracted {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new_with_replace(
                        "Extracted",
                        "pod_ticket_house::ticket_office",
                        &[],
                    ))
                    .type_params(::alloc::vec::Vec::new())
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| f.ty::<AccountId>().name("dest").type_name("AccountId"))
                            .field(|f| f.ty::<Balance>().name("amount").type_name("Balance")),
                    )
            }
        };
    };
    pub struct Recharged {
        account: AccountId,
        amount: Balance,
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Encode for Recharged {
            fn size_hint(&self) -> usize {
                0_usize
                    .saturating_add(::scale::Encode::size_hint(&self.account))
                    .saturating_add(::scale::Encode::size_hint(&self.amount))
            }
            fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::scale::Encode::encode_to(&self.account, __codec_dest_edqy);
                ::scale::Encode::encode_to(&self.amount, __codec_dest_edqy);
            }
        }
        #[automatically_derived]
        impl ::scale::EncodeLike for Recharged {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Decode for Recharged {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(Recharged {
                    account: {
                        let __codec_res_edqy =
                            <AccountId as ::scale::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Recharged::account`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    amount: {
                        let __codec_res_edqy =
                            <Balance as ::scale::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Recharged::amount`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                })
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Recharged {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Recharged",
                "account",
                &self.account,
                "amount",
                &&self.amount,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Recharged {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Recharged {
        #[inline]
        fn eq(&self, other: &Recharged) -> bool {
            self.account == other.account && self.amount == other.amount
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Recharged {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Recharged {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Balance>;
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Recharged {
        #[inline]
        fn clone(&self) -> Recharged {
            Recharged {
                account: ::core::clone::Clone::clone(&self.account),
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for Recharged {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new_with_replace(
                        "Recharged",
                        "pod_ticket_house::ticket_office",
                        &[],
                    ))
                    .type_params(::alloc::vec::Vec::new())
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| f.ty::<AccountId>().name("account").type_name("AccountId"))
                            .field(|f| f.ty::<Balance>().name("amount").type_name("Balance")),
                    )
            }
        };
    };
    const _: () = {
        impl ::ink::env::Topics for Extracted {
            type RemainingTopics = [::ink::env::topics::state::HasRemainingTopics; 1usize];
            fn topics<E, B>(
                &self,
                builder: ::ink::env::topics::TopicsBuilder<::ink::env::topics::state::Uninit, E, B>,
            ) -> <B as ::ink::env::topics::TopicsBuilderBackend<E>>::Output
            where
                E: ::ink::env::Environment,
                B: ::ink::env::topics::TopicsBuilderBackend<E>,
            {
                builder
                    .build::<Self>()
                    .push_topic::<::ink::env::topics::PrefixedValue<[u8; 23usize]>>(
                        &::ink::env::topics::PrefixedValue {
                            value: b"TicketOffice::Extracted",
                            prefix: b"",
                        },
                    )
                    .finish()
            }
        }
    };
    const _: () = {
        impl ::ink::env::Topics for Recharged {
            type RemainingTopics = [::ink::env::topics::state::HasRemainingTopics; 1usize];
            fn topics<E, B>(
                &self,
                builder: ::ink::env::topics::TopicsBuilder<::ink::env::topics::state::Uninit, E, B>,
            ) -> <B as ::ink::env::topics::TopicsBuilderBackend<E>>::Output
            where
                E: ::ink::env::Environment,
                B: ::ink::env::topics::TopicsBuilderBackend<E>,
            {
                builder
                    .build::<Self>()
                    .push_topic::<::ink::env::topics::PrefixedValue<[u8; 23usize]>>(
                        &::ink::env::topics::PrefixedValue {
                            value: b"TicketOffice::Recharged",
                            prefix: b"",
                        },
                    )
                    .finish()
            }
        }
    };
    impl ::ink::reflect::DispatchableConstructorInfo<0xED4B9D1B_u32> for TicketOffice {
        type Input = ();
        type Output = Self;
        type Storage = TicketOffice;
        type Error =
            <::ink::reflect::ConstructorOutputValue<Self> as ::ink::reflect::ConstructorOutput<
                TicketOffice,
            >>::Error;
        const IS_RESULT: ::core::primitive::bool =
            <::ink::reflect::ConstructorOutputValue<Self> as ::ink::reflect::ConstructorOutput<
                TicketOffice,
            >>::IS_RESULT;
        const CALLABLE: fn(Self::Input) -> Self::Output = |_| TicketOffice::default();
        const PAYABLE: ::core::primitive::bool = false;
        const SELECTOR: [::core::primitive::u8; 4usize] = [0xED_u8, 0x4B_u8, 0x9D_u8, 0x1B_u8];
        const LABEL: &'static ::core::primitive::str = "default";
    }
    impl ::ink::reflect::DispatchableMessageInfo<0xFEAEA4FA_u32> for TicketOffice {
        type Input = ();
        type Output = AccountId;
        type Storage = TicketOffice;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output =
            |storage, _| TicketOffice::owner(storage);
        const SELECTOR: [::core::primitive::u8; 4usize] = [0xFE_u8, 0xAE_u8, 0xA4_u8, 0xFA_u8];
        const PAYABLE: ::core::primitive::bool = false;
        const MUTATES: ::core::primitive::bool = false;
        const LABEL: &'static ::core::primitive::str = "owner";
    }
    impl ::ink::reflect::DispatchableMessageInfo<0xEC6D41E1_u32> for TicketOffice {
        type Input = ();
        type Output = this_crate::VersionTuple;
        type Storage = TicketOffice;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output =
            |storage, _| TicketOffice::version(storage);
        const SELECTOR: [::core::primitive::u8; 4usize] = [0xEC_u8, 0x6D_u8, 0x41_u8, 0xE1_u8];
        const PAYABLE: ::core::primitive::bool = false;
        const MUTATES: ::core::primitive::bool = false;
        const LABEL: &'static ::core::primitive::str = "version";
    }
    impl ::ink::reflect::DispatchableMessageInfo<0x94873ED7_u32> for TicketOffice {
        type Input = (AccountId, Balance);
        type Output = Result<()>;
        type Storage = TicketOffice;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output =
            |storage, (__ink_binding_0, __ink_binding_1)| {
                TicketOffice::extract(storage, __ink_binding_0, __ink_binding_1)
            };
        const SELECTOR: [::core::primitive::u8; 4usize] = [0x94_u8, 0x87_u8, 0x3E_u8, 0xD7_u8];
        const PAYABLE: ::core::primitive::bool = false;
        const MUTATES: ::core::primitive::bool = true;
        const LABEL: &'static ::core::primitive::str = "extract";
    }
    impl ::ink::reflect::DispatchableMessageInfo<0x73F4E84A_u32> for TicketOffice {
        type Input = AccountId;
        type Output = Result<()>;
        type Storage = TicketOffice;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output =
            |storage, __ink_binding_0| TicketOffice::recharge(storage, __ink_binding_0);
        const SELECTOR: [::core::primitive::u8; 4usize] = [0x73_u8, 0xF4_u8, 0xE8_u8, 0x4A_u8];
        const PAYABLE: ::core::primitive::bool = true;
        const MUTATES: ::core::primitive::bool = true;
        const LABEL: &'static ::core::primitive::str = "recharge";
    }
    impl ::ink::reflect::DispatchableMessageInfo<0x694FB50F_u32> for TicketOffice {
        type Input = pink::Hash;
        type Output = Result<()>;
        type Storage = TicketOffice;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output =
            |storage, __ink_binding_0| TicketOffice::set_code(storage, __ink_binding_0);
        const SELECTOR: [::core::primitive::u8; 4usize] = [0x69_u8, 0x4F_u8, 0xB5_u8, 0x0F_u8];
        const PAYABLE: ::core::primitive::bool = false;
        const MUTATES: ::core::primitive::bool = true;
        const LABEL: &'static ::core::primitive::str = "set_code";
    }
    const _: () = {
        #[allow(non_camel_case_types)]
        pub enum __ink_ConstructorDecoder {
            Constructor0 (< TicketOffice as :: ink :: reflect :: DispatchableConstructorInfo < 0xED4B9D1B_u32 > > :: Input) , }
        impl ::ink::reflect::DecodeDispatch for __ink_ConstructorDecoder {
            fn decode_dispatch<I>(
                input: &mut I,
            ) -> ::core::result::Result<Self, ::ink::reflect::DispatchError>
            where
                I: ::scale::Input,
            {
                const CONSTRUCTOR_0 : [:: core :: primitive :: u8 ; 4usize] = < TicketOffice as :: ink :: reflect :: DispatchableConstructorInfo < 0xED4B9D1B_u32 > > :: SELECTOR ;
                match <[::core::primitive::u8; 4usize] as ::scale::Decode>::decode(input)
                    .map_err(|_| ::ink::reflect::DispatchError::InvalidSelector)?
                {
                    CONSTRUCTOR_0 => ::core::result::Result::Ok(Self::Constructor0(
                        <<TicketOffice as ::ink::reflect::DispatchableConstructorInfo<
                            0xED4B9D1B_u32,
                        >>::Input as ::scale::Decode>::decode(input)
                        .map_err(|_| ::ink::reflect::DispatchError::InvalidParameters)?,
                    )),
                    _invalid => {
                        ::core::result::Result::Err(::ink::reflect::DispatchError::UnknownSelector)
                    }
                }
            }
        }
        impl ::scale::Decode for __ink_ConstructorDecoder {
            fn decode<I>(input: &mut I) -> ::core::result::Result<Self, ::scale::Error>
            where
                I: ::scale::Input,
            {
                <Self as ::ink::reflect::DecodeDispatch>::decode_dispatch(input)
                    .map_err(::core::convert::Into::into)
            }
        }
        impl ::ink::reflect::ExecuteDispatchable for __ink_ConstructorDecoder {
            #[allow(clippy::nonminimal_bool)]
            fn execute_dispatchable(
                self,
            ) -> ::core::result::Result<(), ::ink::reflect::DispatchError> {
                match self {
                    Self::Constructor0(input) => {
                        if {
                            false || {
                                let constructor_0 = false;
                                let constructor_0 =
                                    <TicketOffice as ::ink::reflect::DispatchableConstructorInfo<
                                        0xED4B9D1B_u32,
                                    >>::PAYABLE;
                                constructor_0
                            }
                        } && !<TicketOffice as ::ink::reflect::DispatchableConstructorInfo<
                            0xED4B9D1B_u32,
                        >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <TicketOffice as ::ink::env::ContractEnv>::Env,
                            >()?;
                        }
                        let result : < TicketOffice as :: ink :: reflect :: DispatchableConstructorInfo < 0xED4B9D1B_u32 > > :: Output = < TicketOffice as :: ink :: reflect :: DispatchableConstructorInfo < 0xED4B9D1B_u32 > > :: CALLABLE (input) ;
                        let output_value = ::ink::reflect::ConstructorOutputValue::new(result);
                        let output_result = <::ink::reflect::ConstructorOutputValue<
                            <TicketOffice as ::ink::reflect::DispatchableConstructorInfo<
                                0xED4B9D1B_u32,
                            >>::Output,
                        > as ::ink::reflect::ConstructorOutput<TicketOffice>>::as_result(
                            &output_value,
                        );
                        if let ::core::result::Result::Ok(contract) = output_result.as_ref() {
                            ::ink::env::set_contract_storage::<::ink::primitives::Key, TicketOffice>(
                                &<TicketOffice as ::ink::storage::traits::StorageKey>::KEY,
                                contract,
                            );
                        }
                        :: ink :: env :: return_value :: < :: ink :: ConstructorResult < :: core :: result :: Result < () , & < :: ink :: reflect :: ConstructorOutputValue < < TicketOffice as :: ink :: reflect :: DispatchableConstructorInfo < 0xED4B9D1B_u32 > > :: Output > as :: ink :: reflect :: ConstructorOutput < TicketOffice > > :: Error > > > (:: ink :: env :: ReturnFlags :: new_with_reverted (output_result . is_err ()) , & :: ink :: ConstructorResult :: Ok (output_result . map (| _ | ()))) ;
                    }
                }
            }
        }
        impl ::ink::reflect::ContractConstructorDecoder for TicketOffice {
            type Type = __ink_ConstructorDecoder;
        }
    };
    const _: () = {
        #[allow(non_camel_case_types)]
        pub enum __ink_MessageDecoder {
            Message0(
                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<0xFEAEA4FA_u32>>::Input,
            ),
            Message1(
                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<0xEC6D41E1_u32>>::Input,
            ),
            Message2(
                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<0x94873ED7_u32>>::Input,
            ),
            Message3(
                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<0x73F4E84A_u32>>::Input,
            ),
            Message4(
                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<0x694FB50F_u32>>::Input,
            ),
        }
        impl ::ink::reflect::DecodeDispatch for __ink_MessageDecoder {
            fn decode_dispatch<I>(
                input: &mut I,
            ) -> ::core::result::Result<Self, ::ink::reflect::DispatchError>
            where
                I: ::scale::Input,
            {
                const MESSAGE_0 : [:: core :: primitive :: u8 ; 4usize] = < TicketOffice as :: ink :: reflect :: DispatchableMessageInfo < 0xFEAEA4FA_u32 > > :: SELECTOR ;
                const MESSAGE_1 : [:: core :: primitive :: u8 ; 4usize] = < TicketOffice as :: ink :: reflect :: DispatchableMessageInfo < 0xEC6D41E1_u32 > > :: SELECTOR ;
                const MESSAGE_2 : [:: core :: primitive :: u8 ; 4usize] = < TicketOffice as :: ink :: reflect :: DispatchableMessageInfo < 0x94873ED7_u32 > > :: SELECTOR ;
                const MESSAGE_3 : [:: core :: primitive :: u8 ; 4usize] = < TicketOffice as :: ink :: reflect :: DispatchableMessageInfo < 0x73F4E84A_u32 > > :: SELECTOR ;
                const MESSAGE_4 : [:: core :: primitive :: u8 ; 4usize] = < TicketOffice as :: ink :: reflect :: DispatchableMessageInfo < 0x694FB50F_u32 > > :: SELECTOR ;
                match <[::core::primitive::u8; 4usize] as ::scale::Decode>::decode(input)
                    .map_err(|_| ::ink::reflect::DispatchError::InvalidSelector)?
                {
                    MESSAGE_0 => {
                        ::core::result::Result::Ok(Self::Message0(
                            <<TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                0xFEAEA4FA_u32,
                            >>::Input as ::scale::Decode>::decode(input)
                            .map_err(|_| ::ink::reflect::DispatchError::InvalidParameters)?,
                        ))
                    }
                    MESSAGE_1 => {
                        ::core::result::Result::Ok(Self::Message1(
                            <<TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                0xEC6D41E1_u32,
                            >>::Input as ::scale::Decode>::decode(input)
                            .map_err(|_| ::ink::reflect::DispatchError::InvalidParameters)?,
                        ))
                    }
                    MESSAGE_2 => {
                        ::core::result::Result::Ok(Self::Message2(
                            <<TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                0x94873ED7_u32,
                            >>::Input as ::scale::Decode>::decode(input)
                            .map_err(|_| ::ink::reflect::DispatchError::InvalidParameters)?,
                        ))
                    }
                    MESSAGE_3 => {
                        ::core::result::Result::Ok(Self::Message3(
                            <<TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                0x73F4E84A_u32,
                            >>::Input as ::scale::Decode>::decode(input)
                            .map_err(|_| ::ink::reflect::DispatchError::InvalidParameters)?,
                        ))
                    }
                    MESSAGE_4 => {
                        ::core::result::Result::Ok(Self::Message4(
                            <<TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                0x694FB50F_u32,
                            >>::Input as ::scale::Decode>::decode(input)
                            .map_err(|_| ::ink::reflect::DispatchError::InvalidParameters)?,
                        ))
                    }
                    _invalid => {
                        ::core::result::Result::Err(::ink::reflect::DispatchError::UnknownSelector)
                    }
                }
            }
        }
        impl ::scale::Decode for __ink_MessageDecoder {
            fn decode<I>(input: &mut I) -> ::core::result::Result<Self, ::scale::Error>
            where
                I: ::scale::Input,
            {
                <Self as ::ink::reflect::DecodeDispatch>::decode_dispatch(input)
                    .map_err(::core::convert::Into::into)
            }
        }
        fn push_contract(contract: ::core::mem::ManuallyDrop<TicketOffice>, mutates: bool) {
            if mutates {
                ::ink::env::set_contract_storage::<::ink::primitives::Key, TicketOffice>(
                    &<TicketOffice as ::ink::storage::traits::StorageKey>::KEY,
                    &contract,
                );
            }
        }
        impl ::ink::reflect::ExecuteDispatchable for __ink_MessageDecoder {
            #[allow(clippy::nonminimal_bool, clippy::let_unit_value)]
            fn execute_dispatchable(
                self,
            ) -> ::core::result::Result<(), ::ink::reflect::DispatchError> {
                let key = <TicketOffice as ::ink::storage::traits::StorageKey>::KEY;
                let mut contract: ::core::mem::ManuallyDrop<TicketOffice> =
                    ::core::mem::ManuallyDrop::new(match ::ink::env::get_contract_storage(&key) {
                        ::core::result::Result::Ok(::core::option::Option::Some(value)) => value,
                        ::core::result::Result::Ok(::core::option::Option::None) => {
                            ::core::panicking::panic_fmt(format_args!("storage entry was empty"));
                        }
                        ::core::result::Result::Err(_) => {
                            ::core::panicking::panic_fmt(format_args!(
                                "could not properly decode storage entry"
                            ));
                        }
                    });
                match self {
                    Self::Message0(input) => {
                        if {
                            false
                                || {
                                    let message_0 = false;
                                    let message_0 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0xFEAEA4FA_u32,
                                        >>::PAYABLE;
                                    message_0
                                }
                                || {
                                    let message_1 = false;
                                    let message_1 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0xEC6D41E1_u32,
                                        >>::PAYABLE;
                                    message_1
                                }
                                || {
                                    let message_2 = false;
                                    let message_2 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0x94873ED7_u32,
                                        >>::PAYABLE;
                                    message_2
                                }
                                || {
                                    let message_3 = false;
                                    let message_3 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0x73F4E84A_u32,
                                        >>::PAYABLE;
                                    message_3
                                }
                                || {
                                    let message_4 = false;
                                    let message_4 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0x694FB50F_u32,
                                        >>::PAYABLE;
                                    message_4
                                }
                        } && !<TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                            0xFEAEA4FA_u32,
                        >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <TicketOffice as ::ink::env::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                            0xFEAEA4FA_u32,
                        >>::Output = <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                            0xFEAEA4FA_u32,
                        >>::CALLABLE(&mut contract, input);
                        let is_reverted = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                    0xFEAEA4FA_u32,
                                >>::Output,
                            >::VALUE
                        } && {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultErrFallback as _;
                            ::ink::result_info::IsResultErr(&result).value()
                        };
                        if !is_reverted {
                            push_contract(
                                contract,
                                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                    0xFEAEA4FA_u32,
                                >>::MUTATES,
                            );
                        }
                        ::ink::env::return_value::<
                            ::ink::MessageResult<
                                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                    0xFEAEA4FA_u32,
                                >>::Output,
                            >,
                        >(
                            ::ink::env::ReturnFlags::new_with_reverted(is_reverted),
                            &::ink::MessageResult::Ok(result),
                        )
                    }
                    Self::Message1(input) => {
                        if {
                            false
                                || {
                                    let message_0 = false;
                                    let message_0 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0xFEAEA4FA_u32,
                                        >>::PAYABLE;
                                    message_0
                                }
                                || {
                                    let message_1 = false;
                                    let message_1 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0xEC6D41E1_u32,
                                        >>::PAYABLE;
                                    message_1
                                }
                                || {
                                    let message_2 = false;
                                    let message_2 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0x94873ED7_u32,
                                        >>::PAYABLE;
                                    message_2
                                }
                                || {
                                    let message_3 = false;
                                    let message_3 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0x73F4E84A_u32,
                                        >>::PAYABLE;
                                    message_3
                                }
                                || {
                                    let message_4 = false;
                                    let message_4 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0x694FB50F_u32,
                                        >>::PAYABLE;
                                    message_4
                                }
                        } && !<TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                            0xEC6D41E1_u32,
                        >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <TicketOffice as ::ink::env::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                            0xEC6D41E1_u32,
                        >>::Output = <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                            0xEC6D41E1_u32,
                        >>::CALLABLE(&mut contract, input);
                        let is_reverted = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                    0xEC6D41E1_u32,
                                >>::Output,
                            >::VALUE
                        } && {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultErrFallback as _;
                            ::ink::result_info::IsResultErr(&result).value()
                        };
                        if !is_reverted {
                            push_contract(
                                contract,
                                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                    0xEC6D41E1_u32,
                                >>::MUTATES,
                            );
                        }
                        ::ink::env::return_value::<
                            ::ink::MessageResult<
                                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                    0xEC6D41E1_u32,
                                >>::Output,
                            >,
                        >(
                            ::ink::env::ReturnFlags::new_with_reverted(is_reverted),
                            &::ink::MessageResult::Ok(result),
                        )
                    }
                    Self::Message2(input) => {
                        if {
                            false
                                || {
                                    let message_0 = false;
                                    let message_0 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0xFEAEA4FA_u32,
                                        >>::PAYABLE;
                                    message_0
                                }
                                || {
                                    let message_1 = false;
                                    let message_1 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0xEC6D41E1_u32,
                                        >>::PAYABLE;
                                    message_1
                                }
                                || {
                                    let message_2 = false;
                                    let message_2 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0x94873ED7_u32,
                                        >>::PAYABLE;
                                    message_2
                                }
                                || {
                                    let message_3 = false;
                                    let message_3 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0x73F4E84A_u32,
                                        >>::PAYABLE;
                                    message_3
                                }
                                || {
                                    let message_4 = false;
                                    let message_4 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0x694FB50F_u32,
                                        >>::PAYABLE;
                                    message_4
                                }
                        } && !<TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                            0x94873ED7_u32,
                        >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <TicketOffice as ::ink::env::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                            0x94873ED7_u32,
                        >>::Output = <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                            0x94873ED7_u32,
                        >>::CALLABLE(&mut contract, input);
                        let is_reverted = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                    0x94873ED7_u32,
                                >>::Output,
                            >::VALUE
                        } && {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultErrFallback as _;
                            ::ink::result_info::IsResultErr(&result).value()
                        };
                        if !is_reverted {
                            push_contract(
                                contract,
                                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                    0x94873ED7_u32,
                                >>::MUTATES,
                            );
                        }
                        ::ink::env::return_value::<
                            ::ink::MessageResult<
                                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                    0x94873ED7_u32,
                                >>::Output,
                            >,
                        >(
                            ::ink::env::ReturnFlags::new_with_reverted(is_reverted),
                            &::ink::MessageResult::Ok(result),
                        )
                    }
                    Self::Message3(input) => {
                        if {
                            false
                                || {
                                    let message_0 = false;
                                    let message_0 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0xFEAEA4FA_u32,
                                        >>::PAYABLE;
                                    message_0
                                }
                                || {
                                    let message_1 = false;
                                    let message_1 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0xEC6D41E1_u32,
                                        >>::PAYABLE;
                                    message_1
                                }
                                || {
                                    let message_2 = false;
                                    let message_2 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0x94873ED7_u32,
                                        >>::PAYABLE;
                                    message_2
                                }
                                || {
                                    let message_3 = false;
                                    let message_3 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0x73F4E84A_u32,
                                        >>::PAYABLE;
                                    message_3
                                }
                                || {
                                    let message_4 = false;
                                    let message_4 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0x694FB50F_u32,
                                        >>::PAYABLE;
                                    message_4
                                }
                        } && !<TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                            0x73F4E84A_u32,
                        >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <TicketOffice as ::ink::env::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                            0x73F4E84A_u32,
                        >>::Output = <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                            0x73F4E84A_u32,
                        >>::CALLABLE(&mut contract, input);
                        let is_reverted = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                    0x73F4E84A_u32,
                                >>::Output,
                            >::VALUE
                        } && {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultErrFallback as _;
                            ::ink::result_info::IsResultErr(&result).value()
                        };
                        if !is_reverted {
                            push_contract(
                                contract,
                                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                    0x73F4E84A_u32,
                                >>::MUTATES,
                            );
                        }
                        ::ink::env::return_value::<
                            ::ink::MessageResult<
                                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                    0x73F4E84A_u32,
                                >>::Output,
                            >,
                        >(
                            ::ink::env::ReturnFlags::new_with_reverted(is_reverted),
                            &::ink::MessageResult::Ok(result),
                        )
                    }
                    Self::Message4(input) => {
                        if {
                            false
                                || {
                                    let message_0 = false;
                                    let message_0 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0xFEAEA4FA_u32,
                                        >>::PAYABLE;
                                    message_0
                                }
                                || {
                                    let message_1 = false;
                                    let message_1 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0xEC6D41E1_u32,
                                        >>::PAYABLE;
                                    message_1
                                }
                                || {
                                    let message_2 = false;
                                    let message_2 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0x94873ED7_u32,
                                        >>::PAYABLE;
                                    message_2
                                }
                                || {
                                    let message_3 = false;
                                    let message_3 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0x73F4E84A_u32,
                                        >>::PAYABLE;
                                    message_3
                                }
                                || {
                                    let message_4 = false;
                                    let message_4 =
                                        <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                            0x694FB50F_u32,
                                        >>::PAYABLE;
                                    message_4
                                }
                        } && !<TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                            0x694FB50F_u32,
                        >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment::<
                                <TicketOffice as ::ink::env::ContractEnv>::Env,
                            >()?;
                        }
                        let result: <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                            0x694FB50F_u32,
                        >>::Output = <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                            0x694FB50F_u32,
                        >>::CALLABLE(&mut contract, input);
                        let is_reverted = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                    0x694FB50F_u32,
                                >>::Output,
                            >::VALUE
                        } && {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultErrFallback as _;
                            ::ink::result_info::IsResultErr(&result).value()
                        };
                        if !is_reverted {
                            push_contract(
                                contract,
                                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                    0x694FB50F_u32,
                                >>::MUTATES,
                            );
                        }
                        ::ink::env::return_value::<
                            ::ink::MessageResult<
                                <TicketOffice as ::ink::reflect::DispatchableMessageInfo<
                                    0x694FB50F_u32,
                                >>::Output,
                            >,
                        >(
                            ::ink::env::ReturnFlags::new_with_reverted(is_reverted),
                            &::ink::MessageResult::Ok(result),
                        )
                    }
                };
            }
        }
        impl ::ink::reflect::ContractMessageDecoder for TicketOffice {
            type Type = __ink_MessageDecoder;
        }
    };
    const _: () = {
        use ::ink::codegen::{Env as _, StaticEnv as _};
        use ::ink::codegen::EmitEvent as _;
        const _: ::ink::codegen::utils::IsSameType<TicketOffice> =
            ::ink::codegen::utils::IsSameType::<TicketOffice>::new();
        impl TicketOffice {
            #[allow(clippy::should_implement_trait)]
            #[cfg(not(feature = "__ink_dylint_Constructor"))]
            pub fn default() -> Self {
                Self {
                    owner: Self::env().caller(),
                }
            }
            pub fn owner(&self) -> AccountId {
                self.owner
            }
            pub fn version(&self) -> this_crate::VersionTuple {
                {
                    let major = "0".parse::<u16>().unwrap_or(0);
                    let minor = "1".parse::<u16>().unwrap_or(0);
                    let patch = "0".parse::<u16>().unwrap_or(0);
                    (major, minor, patch)
                }
            }
            pub fn extract(&mut self, dest: AccountId, amount: Balance) -> Result<()> {
                self.ensure_owner()?;
                self.env()
                    .transfer(dest, amount)
                    .or(Err(Error::TransferFailed))?;
                {
                    {
                        ::pink_extension::logger::log(
                            ::pink_extension::logger::Level::Info,
                            format_args!("Extracted {0} to {1:?}", amount, dest),
                        )
                    }
                };
                self.env().emit_event(Extracted { dest, amount });
                Ok(())
            }
            pub fn recharge(&mut self, account: AccountId) -> Result<()> {
                let amount = self.env().transferred_value();
                if amount == 0 {
                    return Ok(());
                }
                {
                    {
                        ::pink_extension::logger::log(
                            ::pink_extension::logger::Level::Info,
                            format_args!("Recharged {0:?} with {1}.", account, amount),
                        )
                    }
                };
                self.env().emit_event(Recharged { account, amount });
                Ok(())
            }
            #[doc = " For self upgrade."]
            pub fn set_code(&mut self, code_hash: pink::Hash) -> Result<()> {
                self.ensure_owner()?;
                ink::env::set_code_hash(&code_hash).expect("Failed to set code hash");
                {
                    {
                        ::pink_extension::logger::log(
                            ::pink_extension::logger::Level::Info,
                            format_args!("Switched code hash to {0:?}.", code_hash),
                        )
                    }
                };
                Ok(())
            }
            fn ensure_owner(&self) -> Result<()> {
                if self.env().caller() != self.owner {
                    return Err(Error::BadOrigin);
                }
                Ok(())
            }
        }
        const _: () = {
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchOutput<AccountId>>();
            ::ink::codegen::utils::consume_type::<
                ::ink::codegen::DispatchOutput<this_crate::VersionTuple>,
            >();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<AccountId>>();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<Balance>>();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchOutput<Result<()>>>();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<AccountId>>();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchOutput<Result<()>>>();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<pink::Hash>>();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchOutput<Result<()>>>();
        };
    };
    const _: () = {
        #[doc = r" The ink! smart contract's call builder."]
        #[doc = r""]
        #[doc = r" Implements the underlying on-chain calling of the ink! smart contract"]
        #[doc = r" messages and trait implementations in a type safe way."]
        #[repr(transparent)]
        pub struct CallBuilder {
            account_id: AccountId,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CallBuilder {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "CallBuilder",
                    "account_id",
                    &&self.account_id,
                )
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::scale::Encode for CallBuilder {
                fn size_hint(&self) -> usize {
                    ::scale::Encode::size_hint(&&self.account_id)
                }
                fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                    &self,
                    __codec_dest_edqy: &mut __CodecOutputEdqy,
                ) {
                    ::scale::Encode::encode_to(&&self.account_id, __codec_dest_edqy)
                }
                fn encode(&self) -> ::scale::alloc::vec::Vec<::core::primitive::u8> {
                    ::scale::Encode::encode(&&self.account_id)
                }
                fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                    &self,
                    f: F,
                ) -> R {
                    ::scale::Encode::using_encoded(&&self.account_id, f)
                }
            }
            #[automatically_derived]
            impl ::scale::EncodeLike for CallBuilder {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::scale::Decode for CallBuilder {
                fn decode<__CodecInputEdqy: ::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::scale::Error> {
                    ::core::result::Result::Ok(CallBuilder {
                        account_id: {
                            let __codec_res_edqy =
                                <AccountId as ::scale::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `CallBuilder::account_id`"),
                                    )
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    })
                }
                fn decode_into<__CodecInputEdqy: ::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                    dst_: &mut ::core::mem::MaybeUninit<Self>,
                ) -> ::core::result::Result<::scale::DecodeFinished, ::scale::Error>
                {
                    match (
                        &::core::mem::size_of::<AccountId>(),
                        &::core::mem::size_of::<Self>(),
                    ) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    if !(if ::core::mem::size_of::<AccountId>() > 0 {
                        1
                    } else {
                        0
                    } <= 1)
                    {
                        :: core :: panicking :: panic ("assertion failed: if ::core::mem::size_of::<AccountId>() > 0 { 1 } else { 0 } <= 1")
                    };
                    {
                        let dst_: &mut ::core::mem::MaybeUninit<Self> = dst_;
                        let dst_: &mut ::core::mem::MaybeUninit<AccountId> = unsafe {
                            &mut *dst_
                                .as_mut_ptr()
                                .cast::<::core::mem::MaybeUninit<AccountId>>()
                        };
                        <AccountId as ::scale::Decode>::decode_into(__codec_input_edqy, dst_)?;
                    }
                    unsafe {
                        ::core::result::Result::Ok(
                            ::scale::DecodeFinished::assert_decoding_finished(),
                        )
                    }
                }
            }
        };
        #[automatically_derived]
        impl ::core::hash::Hash for CallBuilder {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.account_id, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CallBuilder {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CallBuilder {
            #[inline]
            fn eq(&self, other: &CallBuilder) -> bool {
                self.account_id == other.account_id
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for CallBuilder {}
        #[automatically_derived]
        impl ::core::cmp::Eq for CallBuilder {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<AccountId>;
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CallBuilder {
            #[inline]
            fn clone(&self) -> CallBuilder {
                CallBuilder {
                    account_id: ::core::clone::Clone::clone(&self.account_id),
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl ::scale_info::TypeInfo for CallBuilder {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(::scale_info::Path::new_with_replace(
                            "CallBuilder",
                            "pod_ticket_house::ticket_office",
                            &[],
                        ))
                        .type_params(::alloc::vec::Vec::new())
                        .docs(&[
                            "The ink! smart contract's call builder.",
                            "",
                            "Implements the underlying on-chain calling of the ink! smart contract",
                            "messages and trait implementations in a type safe way.",
                        ])
                        .composite(::scale_info::build::Fields::named().field(|f| {
                            f.ty::<AccountId>()
                                .name("account_id")
                                .type_name("AccountId")
                        }))
                }
            };
        };
        const _: () = {
            impl ::ink::storage::traits::StorageLayout for CallBuilder {
                fn layout(__key: &::ink::primitives::Key) -> ::ink::metadata::layout::Layout {
                    ::ink::metadata::layout::Layout::Struct(
                        ::ink::metadata::layout::StructLayout::new(
                            "CallBuilder",
                            [::ink::metadata::layout::FieldLayout::new(
                                "account_id",
                                <AccountId as ::ink::storage::traits::StorageLayout>::layout(__key),
                            )],
                        ),
                    )
                }
            }
        };
        const _: () = {
            impl ::ink::codegen::ContractCallBuilder for TicketOffice {
                type Type = CallBuilder;
            }
            impl ::ink::env::ContractEnv for CallBuilder {
                type Env = <TicketOffice as ::ink::env::ContractEnv>::Env;
            }
        };
        impl ::ink::env::call::FromAccountId<Environment> for CallBuilder {
            #[inline]
            fn from_account_id(account_id: AccountId) -> Self {
                Self { account_id }
            }
        }
        impl ::ink::ToAccountId<Environment> for CallBuilder {
            #[inline]
            fn to_account_id(&self) -> AccountId {
                <AccountId as ::core::clone::Clone>::clone(&self.account_id)
            }
        }
        impl ::core::convert::AsRef<AccountId> for CallBuilder {
            fn as_ref(&self) -> &AccountId {
                &self.account_id
            }
        }
        impl ::core::convert::AsMut<AccountId> for CallBuilder {
            fn as_mut(&mut self) -> &mut AccountId {
                &mut self.account_id
            }
        }
        impl CallBuilder {
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn owner(
                &self,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Environment>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<::ink::env::call::utils::EmptyArgumentList>,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<AccountId>>,
            > {
                ::ink::env::call::build_call::<Environment>()
                    .call(::ink::ToAccountId::to_account_id(self))
                    .exec_input(::ink::env::call::ExecutionInput::new(
                        ::ink::env::call::Selector::new([0xFE_u8, 0xAE_u8, 0xA4_u8, 0xFA_u8]),
                    ))
                    .returns::<AccountId>()
            }
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn version(
                &self,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Environment>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<::ink::env::call::utils::EmptyArgumentList>,
                >,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::utils::ReturnType<this_crate::VersionTuple>,
                >,
            > {
                ::ink::env::call::build_call::<Environment>()
                    .call(::ink::ToAccountId::to_account_id(self))
                    .exec_input(::ink::env::call::ExecutionInput::new(
                        ::ink::env::call::Selector::new([0xEC_u8, 0x6D_u8, 0x41_u8, 0xE1_u8]),
                    ))
                    .returns::<this_crate::VersionTuple>()
            }
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn extract(
                &mut self,
                __ink_binding_0: AccountId,
                __ink_binding_1: Balance,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Environment>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::ArgumentList<
                            ::ink::env::call::utils::Argument<Balance>,
                            ::ink::env::call::utils::ArgumentList<
                                ::ink::env::call::utils::Argument<AccountId>,
                                ::ink::env::call::utils::EmptyArgumentList,
                            >,
                        >,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<Result<()>>>,
            > {
                ::ink::env::call::build_call::<Environment>()
                    .call(::ink::ToAccountId::to_account_id(self))
                    .exec_input(
                        ::ink::env::call::ExecutionInput::new(::ink::env::call::Selector::new([
                            0x94_u8, 0x87_u8, 0x3E_u8, 0xD7_u8,
                        ]))
                        .push_arg(__ink_binding_0)
                        .push_arg(__ink_binding_1),
                    )
                    .returns::<Result<()>>()
            }
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn recharge(
                &mut self,
                __ink_binding_0: AccountId,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Environment>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::ArgumentList<
                            ::ink::env::call::utils::Argument<AccountId>,
                            ::ink::env::call::utils::EmptyArgumentList,
                        >,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<Result<()>>>,
            > {
                ::ink::env::call::build_call::<Environment>()
                    .call(::ink::ToAccountId::to_account_id(self))
                    .exec_input(
                        ::ink::env::call::ExecutionInput::new(::ink::env::call::Selector::new([
                            0x73_u8, 0xF4_u8, 0xE8_u8, 0x4A_u8,
                        ]))
                        .push_arg(__ink_binding_0),
                    )
                    .returns::<Result<()>>()
            }
            #[doc = " For self upgrade."]
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn set_code(
                &mut self,
                __ink_binding_0: pink::Hash,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call<Environment>>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::ArgumentList<
                            ::ink::env::call::utils::Argument<pink::Hash>,
                            ::ink::env::call::utils::EmptyArgumentList,
                        >,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<Result<()>>>,
            > {
                ::ink::env::call::build_call::<Environment>()
                    .call(::ink::ToAccountId::to_account_id(self))
                    .exec_input(
                        ::ink::env::call::ExecutionInput::new(::ink::env::call::Selector::new([
                            0x69_u8, 0x4F_u8, 0xB5_u8, 0x0F_u8,
                        ]))
                        .push_arg(__ink_binding_0),
                    )
                    .returns::<Result<()>>()
            }
        }
    };
    pub struct TicketOfficeRef {
        inner: <TicketOffice as ::ink::codegen::ContractCallBuilder>::Type,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TicketOfficeRef {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "TicketOfficeRef",
                "inner",
                &&self.inner,
            )
        }
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Encode for TicketOfficeRef {
            fn size_hint(&self) -> usize {
                ::scale::Encode::size_hint(&&self.inner)
            }
            fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::scale::Encode::encode_to(&&self.inner, __codec_dest_edqy)
            }
            fn encode(&self) -> ::scale::alloc::vec::Vec<::core::primitive::u8> {
                ::scale::Encode::encode(&&self.inner)
            }
            fn using_encoded<R, F: ::core::ops::FnOnce(&[::core::primitive::u8]) -> R>(
                &self,
                f: F,
            ) -> R {
                ::scale::Encode::using_encoded(&&self.inner, f)
            }
        }
        #[automatically_derived]
        impl ::scale::EncodeLike for TicketOfficeRef {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Decode for TicketOfficeRef {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                ::core::result::Result::Ok(TicketOfficeRef {
                    inner: {
                        let __codec_res_edqy = < < TicketOffice as :: ink :: codegen :: ContractCallBuilder > :: Type as :: scale :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `TicketOfficeRef::inner`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                })
            }
        }
    };
    #[automatically_derived]
    impl ::core::hash::Hash for TicketOfficeRef {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TicketOfficeRef {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TicketOfficeRef {
        #[inline]
        fn eq(&self, other: &TicketOfficeRef) -> bool {
            self.inner == other.inner
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for TicketOfficeRef {}
    #[automatically_derived]
    impl ::core::cmp::Eq for TicketOfficeRef {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <TicketOffice as ::ink::codegen::ContractCallBuilder>::Type,
            >;
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TicketOfficeRef {
        #[inline]
        fn clone(&self) -> TicketOfficeRef {
            TicketOfficeRef {
                inner: ::core::clone::Clone::clone(&self.inner),
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for TicketOfficeRef {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new_with_replace(
                        "TicketOfficeRef",
                        "pod_ticket_house::ticket_office",
                        &[],
                    ))
                    .type_params(::alloc::vec::Vec::new())
                    .composite(::scale_info::build::Fields::named().field(|f| {
                        f.ty::<<TicketOffice as ::ink::codegen::ContractCallBuilder>::Type>()
                            .name("inner")
                            .type_name("<TicketOffice as::ink::codegen::ContractCallBuilder>::Type")
                    }))
            }
        };
    };
    const _: () = {
        impl ::ink::storage::traits::StorageLayout for TicketOfficeRef {
            fn layout(__key: &::ink::primitives::Key) -> ::ink::metadata::layout::Layout {
                :: ink :: metadata :: layout :: Layout :: Struct (:: ink :: metadata :: layout :: StructLayout :: new ("TicketOfficeRef" , [:: ink :: metadata :: layout :: FieldLayout :: new ("inner" , < < TicketOffice as :: ink :: codegen :: ContractCallBuilder > :: Type as :: ink :: storage :: traits :: StorageLayout > :: layout (__key))]))
            }
        }
    };
    const _: () = {
        impl ::ink::env::ContractReference for TicketOffice {
            type Type = TicketOfficeRef;
        }
        impl ::ink::env::call::ConstructorReturnType<TicketOfficeRef> for TicketOffice {
            type Output = TicketOfficeRef;
            type Error = ();
            fn ok(value: TicketOfficeRef) -> Self::Output {
                value
            }
        }
        impl<E> ::ink::env::call::ConstructorReturnType<TicketOfficeRef>
            for ::core::result::Result<TicketOffice, E>
        where
            E: ::scale::Decode,
        {
            const IS_RESULT: bool = true;
            type Output = ::core::result::Result<TicketOfficeRef, E>;
            type Error = E;
            fn ok(value: TicketOfficeRef) -> Self::Output {
                ::core::result::Result::Ok(value)
            }
            fn err(err: Self::Error) -> ::core::option::Option<Self::Output> {
                ::core::option::Option::Some(::core::result::Result::Err(err))
            }
        }
        impl ::ink::env::ContractEnv for TicketOfficeRef {
            type Env = <TicketOffice as ::ink::env::ContractEnv>::Env;
        }
    };
    impl TicketOfficeRef {
        #[allow(clippy::should_implement_trait)]
        #[inline]
        #[allow(clippy::type_complexity)]
        pub fn default() -> ::ink::env::call::CreateBuilder<
            Environment,
            Self,
            ::ink::env::call::utils::Unset<Hash>,
            ::ink::env::call::utils::Unset<u64>,
            ::ink::env::call::utils::Unset<Balance>,
            ::ink::env::call::utils::Set<
                ::ink::env::call::ExecutionInput<::ink::env::call::utils::EmptyArgumentList>,
            >,
            ::ink::env::call::utils::Unset<::ink::env::call::state::Salt>,
            ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<Self>>,
        > {
            ::ink::env::call::build_create::<Self>()
                .exec_input(::ink::env::call::ExecutionInput::new(
                    ::ink::env::call::Selector::new([0xED_u8, 0x4B_u8, 0x9D_u8, 0x1B_u8]),
                ))
                .returns::<Self>()
        }
        #[inline]
        pub fn owner(&self) -> AccountId {
            self.try_owner().unwrap_or_else(|error| {
                ::core::panicking::panic_fmt(format_args!(
                    "encountered error while calling {0}::{1}: {2:?}",
                    "TicketOffice", "owner", error
                ));
            })
        }
        #[inline]
        pub fn try_owner(&self) -> ::ink::MessageResult<AccountId> {
            <Self as ::ink::codegen::TraitCallBuilder>::call(self)
                .owner()
                .try_invoke()
                .unwrap_or_else(|error| {
                    ::core::panicking::panic_fmt(format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "TicketOffice", "owner", error
                    ));
                })
        }
        #[inline]
        pub fn version(&self) -> this_crate::VersionTuple {
            self.try_version().unwrap_or_else(|error| {
                ::core::panicking::panic_fmt(format_args!(
                    "encountered error while calling {0}::{1}: {2:?}",
                    "TicketOffice", "version", error
                ));
            })
        }
        #[inline]
        pub fn try_version(&self) -> ::ink::MessageResult<this_crate::VersionTuple> {
            <Self as ::ink::codegen::TraitCallBuilder>::call(self)
                .version()
                .try_invoke()
                .unwrap_or_else(|error| {
                    ::core::panicking::panic_fmt(format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "TicketOffice", "version", error
                    ));
                })
        }
        #[inline]
        pub fn extract(&mut self, dest: AccountId, amount: Balance) -> Result<()> {
            self.try_extract(dest, amount).unwrap_or_else(|error| {
                ::core::panicking::panic_fmt(format_args!(
                    "encountered error while calling {0}::{1}: {2:?}",
                    "TicketOffice", "extract", error
                ));
            })
        }
        #[inline]
        pub fn try_extract(
            &mut self,
            dest: AccountId,
            amount: Balance,
        ) -> ::ink::MessageResult<Result<()>> {
            <Self as ::ink::codegen::TraitCallBuilder>::call_mut(self)
                .extract(dest, amount)
                .try_invoke()
                .unwrap_or_else(|error| {
                    ::core::panicking::panic_fmt(format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "TicketOffice", "extract", error
                    ));
                })
        }
        #[inline]
        pub fn recharge(&mut self, account: AccountId) -> Result<()> {
            self.try_recharge(account).unwrap_or_else(|error| {
                ::core::panicking::panic_fmt(format_args!(
                    "encountered error while calling {0}::{1}: {2:?}",
                    "TicketOffice", "recharge", error
                ));
            })
        }
        #[inline]
        pub fn try_recharge(&mut self, account: AccountId) -> ::ink::MessageResult<Result<()>> {
            <Self as ::ink::codegen::TraitCallBuilder>::call_mut(self)
                .recharge(account)
                .try_invoke()
                .unwrap_or_else(|error| {
                    ::core::panicking::panic_fmt(format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "TicketOffice", "recharge", error
                    ));
                })
        }
        #[doc = " For self upgrade."]
        #[inline]
        pub fn set_code(&mut self, code_hash: pink::Hash) -> Result<()> {
            self.try_set_code(code_hash).unwrap_or_else(|error| {
                ::core::panicking::panic_fmt(format_args!(
                    "encountered error while calling {0}::{1}: {2:?}",
                    "TicketOffice", "set_code", error
                ));
            })
        }
        #[doc = " For self upgrade."]
        #[inline]
        pub fn try_set_code(&mut self, code_hash: pink::Hash) -> ::ink::MessageResult<Result<()>> {
            <Self as ::ink::codegen::TraitCallBuilder>::call_mut(self)
                .set_code(code_hash)
                .try_invoke()
                .unwrap_or_else(|error| {
                    ::core::panicking::panic_fmt(format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "TicketOffice", "set_code", error
                    ));
                })
        }
    }
    const _: () = {
        impl ::ink::codegen::TraitCallBuilder for TicketOfficeRef {
            type Builder = <TicketOffice as ::ink::codegen::ContractCallBuilder>::Type;
            #[inline]
            fn call(&self) -> &Self::Builder {
                &self.inner
            }
            #[inline]
            fn call_mut(&mut self) -> &mut Self::Builder {
                &mut self.inner
            }
        }
    };
    impl ::ink::env::call::FromAccountId<Environment> for TicketOfficeRef {
        #[inline]
        fn from_account_id(account_id: AccountId) -> Self {
            Self { inner : < < TicketOffice as :: ink :: codegen :: ContractCallBuilder > :: Type as :: ink :: env :: call :: FromAccountId < Environment > > :: from_account_id (account_id) , }
        }
    }
    impl ::ink::ToAccountId<Environment> for TicketOfficeRef {
        #[inline]
        fn to_account_id(&self) -> AccountId {
            <<TicketOffice as ::ink::codegen::ContractCallBuilder>::Type as ::ink::ToAccountId<
                Environment,
            >>::to_account_id(&self.inner)
        }
    }
    impl ::core::convert::AsRef<AccountId> for TicketOfficeRef {
        fn as_ref(&self) -> &AccountId {
            <_ as ::core::convert::AsRef<AccountId>>::as_ref(&self.inner)
        }
    }
    impl ::core::convert::AsMut<AccountId> for TicketOfficeRef {
        fn as_mut(&mut self) -> &mut AccountId {
            <_ as ::core::convert::AsMut<AccountId>>::as_mut(&mut self.inner)
        }
    }
    #[cfg(feature = "std")]
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        #[no_mangle]
        pub fn __ink_generate_metadata() -> ::ink::metadata::InkProject {
            let layout =
                ::ink::metadata::layout::Layout::Root(::ink::metadata::layout::RootLayout::new(
                    <::ink::metadata::layout::LayoutKey as ::core::convert::From<
                        ::ink::primitives::Key,
                    >>::from(
                        <TicketOffice as ::ink::storage::traits::StorageKey>::KEY
                    ),
                    <TicketOffice as ::ink::storage::traits::StorageLayout>::layout(
                        &<TicketOffice as ::ink::storage::traits::StorageKey>::KEY,
                    ),
                ));
            ::ink::metadata::layout::ValidateLayout::validate(&layout).unwrap_or_else(|error| {
                ::core::panicking::panic_fmt(format_args!(
                    "metadata ink! generation failed: {0}",
                    error
                ));
            });
            :: ink :: metadata :: InkProject :: new (layout , :: ink :: metadata :: ContractSpec :: new () . constructors ([:: ink :: metadata :: ConstructorSpec :: from_label ("default") . selector ([0xED_u8 , 0x4B_u8 , 0x9D_u8 , 0x1B_u8]) . args ([]) . payable (false) . default (false) . returns (:: ink :: metadata :: ReturnTypeSpec :: new (if < TicketOffice as :: ink :: reflect :: DispatchableConstructorInfo < 3981155611u32 > > :: IS_RESULT { :: core :: option :: Option :: Some (:: ink :: metadata :: TypeSpec :: with_name_str :: < :: ink :: ConstructorResult < :: core :: result :: Result < () , < TicketOffice as :: ink :: reflect :: DispatchableConstructorInfo < 3981155611u32 > > :: Error > > > ("ink_primitives::ConstructorResult")) } else { :: core :: option :: Option :: Some (:: ink :: metadata :: TypeSpec :: with_name_str :: < :: ink :: ConstructorResult < () > > ("ink_primitives::ConstructorResult")) })) . docs ([]) . done ()]) . messages ([:: ink :: metadata :: MessageSpec :: from_label ("owner") . selector ([0xFE_u8 , 0xAE_u8 , 0xA4_u8 , 0xFA_u8]) . args ([]) . returns (:: ink :: metadata :: ReturnTypeSpec :: new (:: ink :: metadata :: TypeSpec :: with_name_segs :: < :: ink :: MessageResult < AccountId > , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["ink" , "MessageResult"]) , :: core :: convert :: AsRef :: as_ref)))) . mutates (false) . payable (false) . default (false) . docs ([]) . done () , :: ink :: metadata :: MessageSpec :: from_label ("version") . selector ([0xEC_u8 , 0x6D_u8 , 0x41_u8 , 0xE1_u8]) . args ([]) . returns (:: ink :: metadata :: ReturnTypeSpec :: new (:: ink :: metadata :: TypeSpec :: with_name_segs :: < :: ink :: MessageResult < this_crate :: VersionTuple > , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["ink" , "MessageResult"]) , :: core :: convert :: AsRef :: as_ref)))) . mutates (false) . payable (false) . default (false) . docs ([]) . done () , :: ink :: metadata :: MessageSpec :: from_label ("extract") . selector ([0x94_u8 , 0x87_u8 , 0x3E_u8 , 0xD7_u8]) . args ([:: ink :: metadata :: MessageParamSpec :: new ("dest") . of_type (:: ink :: metadata :: TypeSpec :: with_name_segs :: < AccountId , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["AccountId"]) , :: core :: convert :: AsRef :: as_ref))) . done () , :: ink :: metadata :: MessageParamSpec :: new ("amount") . of_type (:: ink :: metadata :: TypeSpec :: with_name_segs :: < Balance , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["Balance"]) , :: core :: convert :: AsRef :: as_ref))) . done ()]) . returns (:: ink :: metadata :: ReturnTypeSpec :: new (:: ink :: metadata :: TypeSpec :: with_name_segs :: < :: ink :: MessageResult < Result < () > > , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["ink" , "MessageResult"]) , :: core :: convert :: AsRef :: as_ref)))) . mutates (true) . payable (false) . default (false) . docs ([]) . done () , :: ink :: metadata :: MessageSpec :: from_label ("recharge") . selector ([0x73_u8 , 0xF4_u8 , 0xE8_u8 , 0x4A_u8]) . args ([:: ink :: metadata :: MessageParamSpec :: new ("account") . of_type (:: ink :: metadata :: TypeSpec :: with_name_segs :: < AccountId , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["AccountId"]) , :: core :: convert :: AsRef :: as_ref))) . done ()]) . returns (:: ink :: metadata :: ReturnTypeSpec :: new (:: ink :: metadata :: TypeSpec :: with_name_segs :: < :: ink :: MessageResult < Result < () > > , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["ink" , "MessageResult"]) , :: core :: convert :: AsRef :: as_ref)))) . mutates (true) . payable (true) . default (false) . docs ([]) . done () , :: ink :: metadata :: MessageSpec :: from_label ("set_code") . selector ([0x69_u8 , 0x4F_u8 , 0xB5_u8 , 0x0F_u8]) . args ([:: ink :: metadata :: MessageParamSpec :: new ("code_hash") . of_type (:: ink :: metadata :: TypeSpec :: with_name_segs :: < pink :: Hash , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["pink" , "Hash"]) , :: core :: convert :: AsRef :: as_ref))) . done ()]) . returns (:: ink :: metadata :: ReturnTypeSpec :: new (:: ink :: metadata :: TypeSpec :: with_name_segs :: < :: ink :: MessageResult < Result < () > > , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["ink" , "MessageResult"]) , :: core :: convert :: AsRef :: as_ref)))) . mutates (true) . payable (false) . default (false) . docs ([" For self upgrade."]) . done ()]) . events ([:: ink :: metadata :: EventSpec :: new ("Extracted") . args ([:: ink :: metadata :: EventParamSpec :: new ("dest") . of_type (:: ink :: metadata :: TypeSpec :: with_name_segs :: < AccountId , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["AccountId"]) , :: core :: convert :: AsRef :: as_ref))) . indexed (false) . docs ([]) . done () , :: ink :: metadata :: EventParamSpec :: new ("amount") . of_type (:: ink :: metadata :: TypeSpec :: with_name_segs :: < Balance , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["Balance"]) , :: core :: convert :: AsRef :: as_ref))) . indexed (false) . docs ([]) . done ()]) . docs ([]) . done () , :: ink :: metadata :: EventSpec :: new ("Recharged") . args ([:: ink :: metadata :: EventParamSpec :: new ("account") . of_type (:: ink :: metadata :: TypeSpec :: with_name_segs :: < AccountId , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["AccountId"]) , :: core :: convert :: AsRef :: as_ref))) . indexed (false) . docs ([]) . done () , :: ink :: metadata :: EventParamSpec :: new ("amount") . of_type (:: ink :: metadata :: TypeSpec :: with_name_segs :: < Balance , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["Balance"]) , :: core :: convert :: AsRef :: as_ref))) . indexed (false) . docs ([]) . done ()]) . docs ([]) . done ()]) . docs ([]) . lang_error (:: ink :: metadata :: TypeSpec :: with_name_segs :: < :: ink :: LangError , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["ink" , "LangError"]) , :: core :: convert :: AsRef :: as_ref))) . environment (:: ink :: metadata :: EnvironmentSpec :: new () . account_id (:: ink :: metadata :: TypeSpec :: with_name_segs :: < AccountId , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["AccountId"]) , :: core :: convert :: AsRef :: as_ref))) . balance (:: ink :: metadata :: TypeSpec :: with_name_segs :: < Balance , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["Balance"]) , :: core :: convert :: AsRef :: as_ref))) . hash (:: ink :: metadata :: TypeSpec :: with_name_segs :: < Hash , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["Hash"]) , :: core :: convert :: AsRef :: as_ref))) . timestamp (:: ink :: metadata :: TypeSpec :: with_name_segs :: < Timestamp , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["Timestamp"]) , :: core :: convert :: AsRef :: as_ref))) . block_number (:: ink :: metadata :: TypeSpec :: with_name_segs :: < BlockNumber , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["BlockNumber"]) , :: core :: convert :: AsRef :: as_ref))) . chain_extension (:: ink :: metadata :: TypeSpec :: with_name_segs :: < ChainExtension , _ > (:: core :: iter :: Iterator :: map (:: core :: iter :: IntoIterator :: into_iter (["ChainExtension"]) , :: core :: convert :: AsRef :: as_ref))) . max_event_topics (MAX_EVENT_TOPICS) . done ()) . done ())
        }
    };
    use pink::PinkEnvironment;
    type Result<T> = core::result::Result<T, Error>;
    impl Recharged {}
    pub enum Error {
        BadOrigin,
        TransferFailed,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl ::scale_info::TypeInfo for Error {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new_with_replace(
                        "Error",
                        "pod_ticket_house::ticket_office",
                        &[],
                    ))
                    .type_params(::alloc::vec::Vec::new())
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant("BadOrigin", |v| v.index(0usize as ::core::primitive::u8))
                            .variant("TransferFailed", |v| {
                                v.index(1usize as ::core::primitive::u8)
                            }),
                    )
            }
        };
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Error::BadOrigin => "BadOrigin",
                    Error::TransferFailed => "TransferFailed",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Error {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Error {
        #[inline]
        fn eq(&self, other: &Error) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Error {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Error {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Encode for Error {
            fn size_hint(&self) -> usize {
                1_usize
                    + match *self {
                        Error::BadOrigin => 0_usize,
                        Error::TransferFailed => 0_usize,
                        _ => 0_usize,
                    }
            }
            fn encode_to<__CodecOutputEdqy: ::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Error::BadOrigin => {
                        #[allow(clippy::unnecessary_cast)]
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                    }
                    Error::TransferFailed => {
                        #[allow(clippy::unnecessary_cast)]
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                    }
                    _ => (),
                }
            }
        }
        #[automatically_derived]
        impl ::scale::EncodeLike for Error {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl ::scale::Decode for Error {
            fn decode<__CodecInputEdqy: ::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::scale::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Error`, failed to read variant byte"))?
                {
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || ::core::result::Result::Ok(Error::BadOrigin))();
                    }
                    #[allow(clippy::unnecessary_cast)]
                    __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || ::core::result::Result::Ok(Error::TransferFailed))();
                    }
                    _ => {
                        #[allow(clippy::redundant_closure_call)]
                        return (move || {
                            ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                                "Could not decode `Error`, variant doesn't exist",
                            ))
                        })();
                    }
                }
            }
        }
    };
}
