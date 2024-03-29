// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `proto/request_1.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:my_request.RequestMessage)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RequestMessage {
    // message fields
    // @@protoc_insertion_point(field:my_request.RequestMessage.message)
    pub message: ::std::string::String,
    // @@protoc_insertion_point(field:my_request.RequestMessage.code)
    pub code: u32,
    // special fields
    // @@protoc_insertion_point(special_field:my_request.RequestMessage.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RequestMessage {
    fn default() -> &'a RequestMessage {
        <RequestMessage as ::protobuf::Message>::default_instance()
    }
}

impl RequestMessage {
    pub fn new() -> RequestMessage {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "message",
            |m: &RequestMessage| { &m.message },
            |m: &mut RequestMessage| { &mut m.message },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "code",
            |m: &RequestMessage| { &m.code },
            |m: &mut RequestMessage| { &mut m.code },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RequestMessage>(
            "RequestMessage",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RequestMessage {
    const NAME: &'static str = "RequestMessage";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.message = is.read_string()?;
                },
                16 => {
                    self.code = is.read_uint32()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        }
        if self.code != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.code);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.message.is_empty() {
            os.write_string(1, &self.message)?;
        }
        if self.code != 0 {
            os.write_uint32(2, self.code)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> RequestMessage {
        RequestMessage::new()
    }

    fn clear(&mut self) {
        self.message.clear();
        self.code = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RequestMessage {
        static instance: RequestMessage = RequestMessage {
            message: ::std::string::String::new(),
            code: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RequestMessage {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RequestMessage").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RequestMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestMessage {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15proto/request_1.proto\x12\nmy_request\">\n\x0eRequestMessage\x12\
    \x18\n\x07message\x18\x01\x20\x01(\tR\x07message\x12\x12\n\x04code\x18\
    \x02\x20\x01(\rR\x04codeJ\xa2\x01\n\x06\x12\x04\0\0\x06\x01\n\x08\n\x01\
    \x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\0\x13\n\n\n\x02\x04\0\
    \x12\x04\x03\0\x06\x01\n\n\n\x03\x04\0\x01\x12\x03\x03\x08\x16\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\x04\x04\x17\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\
    \x04\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x04\x0b\x12\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\x04\x15\x16\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \x05\x04\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x05\x04\n\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x05\x0b\x0f\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x05\x12\x13b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RequestMessage::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
