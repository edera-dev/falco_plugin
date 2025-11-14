// @generated
impl serde::Serialize for AnnotationSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.AnnotationSpec", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AnnotationSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AnnotationSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.AnnotationSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AnnotationSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AnnotationSpec {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.AnnotationSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConfigureZoneNetworkReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.ConfigureZoneNetworkReply", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfigureZoneNetworkReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigureZoneNetworkReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ConfigureZoneNetworkReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConfigureZoneNetworkReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ConfigureZoneNetworkReply {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ConfigureZoneNetworkReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConfigureZoneNetworkRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_id.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ConfigureZoneNetworkRequest", len)?;
        if !self.zone_id.is_empty() {
            struct_ser.serialize_field("zoneId", &self.zone_id)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfigureZoneNetworkRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_id",
            "zoneId",
            "config",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneId,
            Config,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigureZoneNetworkRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ConfigureZoneNetworkRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConfigureZoneNetworkRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_id__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ConfigureZoneNetworkRequest {
                    zone_id: zone_id__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ConfigureZoneNetworkRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateNetworkReservationReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reservation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.CreateNetworkReservationReply", len)?;
        if let Some(v) = self.reservation.as_ref() {
            struct_ser.serialize_field("reservation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateNetworkReservationReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reservation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reservation,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reservation" => Ok(GeneratedField::Reservation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateNetworkReservationReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.CreateNetworkReservationReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateNetworkReservationReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reservation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reservation => {
                            if reservation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservation"));
                            }
                            reservation__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateNetworkReservationReply {
                    reservation: reservation__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.CreateNetworkReservationReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateNetworkReservationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.CreateNetworkReservationRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateNetworkReservationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateNetworkReservationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.CreateNetworkReservationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateNetworkReservationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CreateNetworkReservationRequest {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.CreateNetworkReservationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWorkloadReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.workload_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.CreateWorkloadReply", len)?;
        if !self.workload_id.is_empty() {
            struct_ser.serialize_field("workloadId", &self.workload_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWorkloadReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workload_id",
            "workloadId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkloadId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "workloadId" | "workload_id" => Ok(GeneratedField::WorkloadId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateWorkloadReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.CreateWorkloadReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateWorkloadReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workload_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkloadId => {
                            if workload_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workloadId"));
                            }
                            workload_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateWorkloadReply {
                    workload_id: workload_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.CreateWorkloadReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateWorkloadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.CreateWorkloadRequest", len)?;
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateWorkloadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "spec",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Spec,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "spec" => Ok(GeneratedField::Spec),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateWorkloadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.CreateWorkloadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateWorkloadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut spec__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateWorkloadRequest {
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.CreateWorkloadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateZoneReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.CreateZoneReply", len)?;
        if !self.zone_id.is_empty() {
            struct_ser.serialize_field("zoneId", &self.zone_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateZoneReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_id",
            "zoneId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateZoneReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.CreateZoneReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateZoneReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(CreateZoneReply {
                    zone_id: zone_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.CreateZoneReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateZoneRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.CreateZoneRequest", len)?;
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateZoneRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "spec",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Spec,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "spec" => Ok(GeneratedField::Spec),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateZoneRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.CreateZoneRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateZoneRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut spec__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateZoneRequest {
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.CreateZoneRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DestroyNetworkReservationReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.DestroyNetworkReservationReply", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DestroyNetworkReservationReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DestroyNetworkReservationReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.DestroyNetworkReservationReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DestroyNetworkReservationReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DestroyNetworkReservationReply {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.DestroyNetworkReservationReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DestroyNetworkReservationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reservation_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.DestroyNetworkReservationRequest", len)?;
        if !self.reservation_id.is_empty() {
            struct_ser.serialize_field("reservationId", &self.reservation_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DestroyNetworkReservationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reservation_id",
            "reservationId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReservationId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reservationId" | "reservation_id" => Ok(GeneratedField::ReservationId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DestroyNetworkReservationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.DestroyNetworkReservationRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DestroyNetworkReservationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reservation_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReservationId => {
                            if reservation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservationId"));
                            }
                            reservation_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DestroyNetworkReservationRequest {
                    reservation_id: reservation_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.DestroyNetworkReservationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DestroyWorkloadReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.DestroyWorkloadReply", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DestroyWorkloadReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DestroyWorkloadReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.DestroyWorkloadReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DestroyWorkloadReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DestroyWorkloadReply {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.DestroyWorkloadReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DestroyWorkloadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.workload_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.DestroyWorkloadRequest", len)?;
        if !self.workload_id.is_empty() {
            struct_ser.serialize_field("workloadId", &self.workload_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DestroyWorkloadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workload_id",
            "workloadId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkloadId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "workloadId" | "workload_id" => Ok(GeneratedField::WorkloadId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DestroyWorkloadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.DestroyWorkloadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DestroyWorkloadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workload_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkloadId => {
                            if workload_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workloadId"));
                            }
                            workload_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DestroyWorkloadRequest {
                    workload_id: workload_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.DestroyWorkloadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DestroyZoneReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.DestroyZoneReply", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DestroyZoneReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DestroyZoneReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.DestroyZoneReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DestroyZoneReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DestroyZoneReply {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.DestroyZoneReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DestroyZoneRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.DestroyZoneRequest", len)?;
        if !self.zone_id.is_empty() {
            struct_ser.serialize_field("zoneId", &self.zone_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DestroyZoneRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_id",
            "zoneId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DestroyZoneRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.DestroyZoneRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DestroyZoneRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DestroyZoneRequest {
                    zone_id: zone_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.DestroyZoneRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeviceInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.claimed {
            len += 1;
        }
        if !self.owner_zone.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.DeviceInfo", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.claimed {
            struct_ser.serialize_field("claimed", &self.claimed)?;
        }
        if !self.owner_zone.is_empty() {
            struct_ser.serialize_field("ownerZone", &self.owner_zone)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeviceInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "claimed",
            "owner_zone",
            "ownerZone",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Claimed,
            OwnerZone,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "claimed" => Ok(GeneratedField::Claimed),
                            "ownerZone" | "owner_zone" => Ok(GeneratedField::OwnerZone),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeviceInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.DeviceInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeviceInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut claimed__ = None;
                let mut owner_zone__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Claimed => {
                            if claimed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("claimed"));
                            }
                            claimed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OwnerZone => {
                            if owner_zone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ownerZone"));
                            }
                            owner_zone__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeviceInfo {
                    name: name__.unwrap_or_default(),
                    claimed: claimed__.unwrap_or_default(),
                    owner_zone: owner_zone__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.DeviceInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeviceReferenceSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.DeviceReferenceSpec", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeviceReferenceSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeviceReferenceSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.DeviceReferenceSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeviceReferenceSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeviceReferenceSpec {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.DeviceReferenceSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DialNetworkSocketData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.content.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.DialNetworkSocketData", len)?;
        if !self.content.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("content", pbjson::private::base64::encode(&self.content).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DialNetworkSocketData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "content",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Content,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "content" => Ok(GeneratedField::Content),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DialNetworkSocketData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.DialNetworkSocketData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DialNetworkSocketData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut content__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DialNetworkSocketData {
                    content: content__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.DialNetworkSocketData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DialNetworkSocketReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.DialNetworkSocketReply", len)?;
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DialNetworkSocketReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DialNetworkSocketReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.DialNetworkSocketReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DialNetworkSocketReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DialNetworkSocketReply {
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.DialNetworkSocketReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DialNetworkSocketRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.DialNetworkSocketRequest", len)?;
        if let Some(v) = self.request.as_ref() {
            match v {
                dial_network_socket_request::Request::Start(v) => {
                    struct_ser.serialize_field("start", v)?;
                }
                dial_network_socket_request::Request::Data(v) => {
                    struct_ser.serialize_field("data", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DialNetworkSocketRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start",
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            Data,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "start" => Ok(GeneratedField::Start),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DialNetworkSocketRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.DialNetworkSocketRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DialNetworkSocketRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(dial_network_socket_request::Request::Start)
;
                        }
                        GeneratedField::Data => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(dial_network_socket_request::Request::Data)
;
                        }
                    }
                }
                Ok(DialNetworkSocketRequest {
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.DialNetworkSocketRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DialNetworkSocketStart {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_id.is_empty() {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        if self.port != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.DialNetworkSocketStart", len)?;
        if !self.zone_id.is_empty() {
            struct_ser.serialize_field("zoneId", &self.zone_id)?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.port != 0 {
            struct_ser.serialize_field("port", &self.port)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DialNetworkSocketStart {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_id",
            "zoneId",
            "address",
            "port",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneId,
            Address,
            Port,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            "address" => Ok(GeneratedField::Address),
                            "port" => Ok(GeneratedField::Port),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DialNetworkSocketStart;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.DialNetworkSocketStart")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DialNetworkSocketStart, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_id__ = None;
                let mut address__ = None;
                let mut port__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DialNetworkSocketStart {
                    zone_id: zone_id__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    port: port__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.DialNetworkSocketStart", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnvironmentVariableSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.EnvironmentVariableSpec", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnvironmentVariableSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnvironmentVariableSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.EnvironmentVariableSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EnvironmentVariableSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EnvironmentVariableSpec {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.EnvironmentVariableSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecuteZoneCommandExited {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.exit_code != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ExecuteZoneCommandExited", len)?;
        if self.exit_code != 0 {
            struct_ser.serialize_field("exitCode", &self.exit_code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecuteZoneCommandExited {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "exit_code",
            "exitCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExitCode,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "exitCode" | "exit_code" => Ok(GeneratedField::ExitCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecuteZoneCommandExited;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ExecuteZoneCommandExited")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecuteZoneCommandExited, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut exit_code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExitCode => {
                            if exit_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exitCode"));
                            }
                            exit_code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ExecuteZoneCommandExited {
                    exit_code: exit_code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ExecuteZoneCommandExited", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecuteZoneCommandOutput {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stdout.is_empty() {
            len += 1;
        }
        if !self.stderr.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ExecuteZoneCommandOutput", len)?;
        if !self.stdout.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("stdout", pbjson::private::base64::encode(&self.stdout).as_str())?;
        }
        if !self.stderr.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("stderr", pbjson::private::base64::encode(&self.stderr).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecuteZoneCommandOutput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stdout",
            "stderr",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Stdout,
            Stderr,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stdout" => Ok(GeneratedField::Stdout),
                            "stderr" => Ok(GeneratedField::Stderr),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecuteZoneCommandOutput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ExecuteZoneCommandOutput")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecuteZoneCommandOutput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stdout__ = None;
                let mut stderr__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Stdout => {
                            if stdout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdout"));
                            }
                            stdout__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Stderr => {
                            if stderr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stderr"));
                            }
                            stderr__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ExecuteZoneCommandOutput {
                    stdout: stdout__.unwrap_or_default(),
                    stderr: stderr__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ExecuteZoneCommandOutput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecuteZoneCommandReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reply.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ExecuteZoneCommandReply", len)?;
        if let Some(v) = self.reply.as_ref() {
            match v {
                execute_zone_command_reply::Reply::Output(v) => {
                    struct_ser.serialize_field("output", v)?;
                }
                execute_zone_command_reply::Reply::Exited(v) => {
                    struct_ser.serialize_field("exited", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecuteZoneCommandReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "output",
            "exited",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Output,
            Exited,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "output" => Ok(GeneratedField::Output),
                            "exited" => Ok(GeneratedField::Exited),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecuteZoneCommandReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ExecuteZoneCommandReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecuteZoneCommandReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reply__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Output => {
                            if reply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("output"));
                            }
                            reply__ = map_.next_value::<::std::option::Option<_>>()?.map(execute_zone_command_reply::Reply::Output)
;
                        }
                        GeneratedField::Exited => {
                            if reply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exited"));
                            }
                            reply__ = map_.next_value::<::std::option::Option<_>>()?.map(execute_zone_command_reply::Reply::Exited)
;
                        }
                    }
                }
                Ok(ExecuteZoneCommandReply {
                    reply: reply__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ExecuteZoneCommandReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecuteZoneCommandRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ExecuteZoneCommandRequest", len)?;
        if let Some(v) = self.request.as_ref() {
            match v {
                execute_zone_command_request::Request::Start(v) => {
                    struct_ser.serialize_field("start", v)?;
                }
                execute_zone_command_request::Request::Stdin(v) => {
                    struct_ser.serialize_field("stdin", v)?;
                }
                execute_zone_command_request::Request::TerminalResize(v) => {
                    struct_ser.serialize_field("terminalResize", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecuteZoneCommandRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start",
            "stdin",
            "terminal_resize",
            "terminalResize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            Stdin,
            TerminalResize,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "start" => Ok(GeneratedField::Start),
                            "stdin" => Ok(GeneratedField::Stdin),
                            "terminalResize" | "terminal_resize" => Ok(GeneratedField::TerminalResize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecuteZoneCommandRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ExecuteZoneCommandRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecuteZoneCommandRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(execute_zone_command_request::Request::Start)
;
                        }
                        GeneratedField::Stdin => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdin"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(execute_zone_command_request::Request::Stdin)
;
                        }
                        GeneratedField::TerminalResize => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("terminalResize"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(execute_zone_command_request::Request::TerminalResize)
;
                        }
                    }
                }
                Ok(ExecuteZoneCommandRequest {
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ExecuteZoneCommandRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecuteZoneCommandStart {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_id.is_empty() {
            len += 1;
        }
        if !self.workload_id.is_empty() {
            len += 1;
        }
        if self.process.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ExecuteZoneCommandStart", len)?;
        if !self.zone_id.is_empty() {
            struct_ser.serialize_field("zoneId", &self.zone_id)?;
        }
        if !self.workload_id.is_empty() {
            struct_ser.serialize_field("workloadId", &self.workload_id)?;
        }
        if let Some(v) = self.process.as_ref() {
            struct_ser.serialize_field("process", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecuteZoneCommandStart {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_id",
            "zoneId",
            "workload_id",
            "workloadId",
            "process",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneId,
            WorkloadId,
            Process,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            "workloadId" | "workload_id" => Ok(GeneratedField::WorkloadId),
                            "process" => Ok(GeneratedField::Process),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecuteZoneCommandStart;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ExecuteZoneCommandStart")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecuteZoneCommandStart, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_id__ = None;
                let mut workload_id__ = None;
                let mut process__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WorkloadId => {
                            if workload_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workloadId"));
                            }
                            workload_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Process => {
                            if process__.is_some() {
                                return Err(serde::de::Error::duplicate_field("process"));
                            }
                            process__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ExecuteZoneCommandStart {
                    zone_id: zone_id__.unwrap_or_default(),
                    workload_id: workload_id__.unwrap_or_default(),
                    process: process__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ExecuteZoneCommandStart", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecuteZoneCommandStdin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stdin.is_empty() {
            len += 1;
        }
        if self.closed {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ExecuteZoneCommandStdin", len)?;
        if !self.stdin.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("stdin", pbjson::private::base64::encode(&self.stdin).as_str())?;
        }
        if self.closed {
            struct_ser.serialize_field("closed", &self.closed)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecuteZoneCommandStdin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stdin",
            "closed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Stdin,
            Closed,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stdin" => Ok(GeneratedField::Stdin),
                            "closed" => Ok(GeneratedField::Closed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecuteZoneCommandStdin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ExecuteZoneCommandStdin")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecuteZoneCommandStdin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stdin__ = None;
                let mut closed__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Stdin => {
                            if stdin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdin"));
                            }
                            stdin__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Closed => {
                            if closed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closed"));
                            }
                            closed__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExecuteZoneCommandStdin {
                    stdin: stdin__.unwrap_or_default(),
                    closed: closed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ExecuteZoneCommandStdin", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecuteZoneCommandTerminalResize {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ExecuteZoneCommandTerminalResize", len)?;
        if let Some(v) = self.size.as_ref() {
            struct_ser.serialize_field("size", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecuteZoneCommandTerminalResize {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "size",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Size,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "size" => Ok(GeneratedField::Size),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecuteZoneCommandTerminalResize;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ExecuteZoneCommandTerminalResize")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecuteZoneCommandTerminalResize, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ExecuteZoneCommandTerminalResize {
                    size: size__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ExecuteZoneCommandTerminalResize", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetHostCpuTopologyReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cpus.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.GetHostCpuTopologyReply", len)?;
        if !self.cpus.is_empty() {
            struct_ser.serialize_field("cpus", &self.cpus)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetHostCpuTopologyReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cpus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cpus,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cpus" => Ok(GeneratedField::Cpus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetHostCpuTopologyReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.GetHostCpuTopologyReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetHostCpuTopologyReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cpus__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cpus => {
                            if cpus__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpus"));
                            }
                            cpus__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetHostCpuTopologyReply {
                    cpus: cpus__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.GetHostCpuTopologyReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetHostCpuTopologyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.GetHostCpuTopologyRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetHostCpuTopologyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetHostCpuTopologyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.GetHostCpuTopologyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetHostCpuTopologyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetHostCpuTopologyRequest {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.GetHostCpuTopologyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetHostStatusReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.host_uuid.is_empty() {
            len += 1;
        }
        if self.host_domid != 0 {
            len += 1;
        }
        if !self.protect_version.is_empty() {
            len += 1;
        }
        if !self.host_ipv4.is_empty() {
            len += 1;
        }
        if !self.host_ipv6.is_empty() {
            len += 1;
        }
        if !self.host_mac.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.GetHostStatusReply", len)?;
        if !self.host_uuid.is_empty() {
            struct_ser.serialize_field("hostUuid", &self.host_uuid)?;
        }
        if self.host_domid != 0 {
            struct_ser.serialize_field("hostDomid", &self.host_domid)?;
        }
        if !self.protect_version.is_empty() {
            struct_ser.serialize_field("protectVersion", &self.protect_version)?;
        }
        if !self.host_ipv4.is_empty() {
            struct_ser.serialize_field("hostIpv4", &self.host_ipv4)?;
        }
        if !self.host_ipv6.is_empty() {
            struct_ser.serialize_field("hostIpv6", &self.host_ipv6)?;
        }
        if !self.host_mac.is_empty() {
            struct_ser.serialize_field("hostMac", &self.host_mac)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetHostStatusReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host_uuid",
            "hostUuid",
            "host_domid",
            "hostDomid",
            "protect_version",
            "protectVersion",
            "host_ipv4",
            "hostIpv4",
            "host_ipv6",
            "hostIpv6",
            "host_mac",
            "hostMac",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostUuid,
            HostDomid,
            ProtectVersion,
            HostIpv4,
            HostIpv6,
            HostMac,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hostUuid" | "host_uuid" => Ok(GeneratedField::HostUuid),
                            "hostDomid" | "host_domid" => Ok(GeneratedField::HostDomid),
                            "protectVersion" | "protect_version" => Ok(GeneratedField::ProtectVersion),
                            "hostIpv4" | "host_ipv4" => Ok(GeneratedField::HostIpv4),
                            "hostIpv6" | "host_ipv6" => Ok(GeneratedField::HostIpv6),
                            "hostMac" | "host_mac" => Ok(GeneratedField::HostMac),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetHostStatusReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.GetHostStatusReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetHostStatusReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host_uuid__ = None;
                let mut host_domid__ = None;
                let mut protect_version__ = None;
                let mut host_ipv4__ = None;
                let mut host_ipv6__ = None;
                let mut host_mac__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HostUuid => {
                            if host_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostUuid"));
                            }
                            host_uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HostDomid => {
                            if host_domid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostDomid"));
                            }
                            host_domid__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProtectVersion => {
                            if protect_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protectVersion"));
                            }
                            protect_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HostIpv4 => {
                            if host_ipv4__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostIpv4"));
                            }
                            host_ipv4__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HostIpv6 => {
                            if host_ipv6__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostIpv6"));
                            }
                            host_ipv6__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HostMac => {
                            if host_mac__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostMac"));
                            }
                            host_mac__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetHostStatusReply {
                    host_uuid: host_uuid__.unwrap_or_default(),
                    host_domid: host_domid__.unwrap_or_default(),
                    protect_version: protect_version__.unwrap_or_default(),
                    host_ipv4: host_ipv4__.unwrap_or_default(),
                    host_ipv6: host_ipv6__.unwrap_or_default(),
                    host_mac: host_mac__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.GetHostStatusReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetHostStatusRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.GetHostStatusRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetHostStatusRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetHostStatusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.GetHostStatusRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetHostStatusRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetHostStatusRequest {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.GetHostStatusRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetWorkloadReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.workload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.GetWorkloadReply", len)?;
        if let Some(v) = self.workload.as_ref() {
            struct_ser.serialize_field("workload", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWorkloadReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workload",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Workload,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "workload" => Ok(GeneratedField::Workload),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetWorkloadReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.GetWorkloadReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWorkloadReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Workload => {
                            if workload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workload"));
                            }
                            workload__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetWorkloadReply {
                    workload: workload__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.GetWorkloadReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetWorkloadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.workload_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.GetWorkloadRequest", len)?;
        if !self.workload_id.is_empty() {
            struct_ser.serialize_field("workloadId", &self.workload_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetWorkloadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workload_id",
            "workloadId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkloadId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "workloadId" | "workload_id" => Ok(GeneratedField::WorkloadId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetWorkloadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.GetWorkloadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetWorkloadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workload_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkloadId => {
                            if workload_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workloadId"));
                            }
                            workload_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetWorkloadRequest {
                    workload_id: workload_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.GetWorkloadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetZoneReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.zone.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.GetZoneReply", len)?;
        if let Some(v) = self.zone.as_ref() {
            struct_ser.serialize_field("zone", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetZoneReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Zone,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zone" => Ok(GeneratedField::Zone),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetZoneReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.GetZoneReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetZoneReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Zone => {
                            if zone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zone"));
                            }
                            zone__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetZoneReply {
                    zone: zone__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.GetZoneReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetZoneRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.GetZoneRequest", len)?;
        if !self.zone_id.is_empty() {
            struct_ser.serialize_field("zoneId", &self.zone_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetZoneRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_id",
            "zoneId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetZoneRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.GetZoneRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetZoneRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetZoneRequest {
                    zone_id: zone_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.GetZoneRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HostCpuTopologyClass {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Standard => "HOST_CPU_TOPOLOGY_CLASS_STANDARD",
            Self::Performance => "HOST_CPU_TOPOLOGY_CLASS_PERFORMANCE",
            Self::Efficiency => "HOST_CPU_TOPOLOGY_CLASS_EFFICIENCY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for HostCpuTopologyClass {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "HOST_CPU_TOPOLOGY_CLASS_STANDARD",
            "HOST_CPU_TOPOLOGY_CLASS_PERFORMANCE",
            "HOST_CPU_TOPOLOGY_CLASS_EFFICIENCY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostCpuTopologyClass;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "HOST_CPU_TOPOLOGY_CLASS_STANDARD" => Ok(HostCpuTopologyClass::Standard),
                    "HOST_CPU_TOPOLOGY_CLASS_PERFORMANCE" => Ok(HostCpuTopologyClass::Performance),
                    "HOST_CPU_TOPOLOGY_CLASS_EFFICIENCY" => Ok(HostCpuTopologyClass::Efficiency),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for HostCpuTopologyInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.core != 0 {
            len += 1;
        }
        if self.socket != 0 {
            len += 1;
        }
        if self.node != 0 {
            len += 1;
        }
        if self.thread != 0 {
            len += 1;
        }
        if self.class != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.HostCpuTopologyInfo", len)?;
        if self.core != 0 {
            struct_ser.serialize_field("core", &self.core)?;
        }
        if self.socket != 0 {
            struct_ser.serialize_field("socket", &self.socket)?;
        }
        if self.node != 0 {
            struct_ser.serialize_field("node", &self.node)?;
        }
        if self.thread != 0 {
            struct_ser.serialize_field("thread", &self.thread)?;
        }
        if self.class != 0 {
            let v = HostCpuTopologyClass::try_from(self.class)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.class)))?;
            struct_ser.serialize_field("class", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostCpuTopologyInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "core",
            "socket",
            "node",
            "thread",
            "class",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Core,
            Socket,
            Node,
            Thread,
            Class,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "core" => Ok(GeneratedField::Core),
                            "socket" => Ok(GeneratedField::Socket),
                            "node" => Ok(GeneratedField::Node),
                            "thread" => Ok(GeneratedField::Thread),
                            "class" => Ok(GeneratedField::Class),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostCpuTopologyInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.HostCpuTopologyInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HostCpuTopologyInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut core__ = None;
                let mut socket__ = None;
                let mut node__ = None;
                let mut thread__ = None;
                let mut class__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Core => {
                            if core__.is_some() {
                                return Err(serde::de::Error::duplicate_field("core"));
                            }
                            core__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Socket => {
                            if socket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("socket"));
                            }
                            socket__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Thread => {
                            if thread__.is_some() {
                                return Err(serde::de::Error::duplicate_field("thread"));
                            }
                            thread__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Class => {
                            if class__.is_some() {
                                return Err(serde::de::Error::duplicate_field("class"));
                            }
                            class__ = Some(map_.next_value::<HostCpuTopologyClass>()? as i32);
                        }
                    }
                }
                Ok(HostCpuTopologyInfo {
                    core: core__.unwrap_or_default(),
                    socket: socket__.unwrap_or_default(),
                    node: node__.unwrap_or_default(),
                    thread: thread__.unwrap_or_default(),
                    class: class__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.HostCpuTopologyInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HypervisorDebugInfoReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.json.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.HypervisorDebugInfoReply", len)?;
        if !self.json.is_empty() {
            struct_ser.serialize_field("json", &self.json)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HypervisorDebugInfoReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "json",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Json,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "json" => Ok(GeneratedField::Json),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HypervisorDebugInfoReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.HypervisorDebugInfoReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HypervisorDebugInfoReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut json__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Json => {
                            if json__.is_some() {
                                return Err(serde::de::Error::duplicate_field("json"));
                            }
                            json__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(HypervisorDebugInfoReply {
                    json: json__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.HypervisorDebugInfoReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HypervisorDebugInfoRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.HypervisorDebugInfoRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HypervisorDebugInfoRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HypervisorDebugInfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.HypervisorDebugInfoRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<HypervisorDebugInfoRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(HypervisorDebugInfoRequest {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.HypervisorDebugInfoRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdOrNameValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id_or_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.IdOrNameValue", len)?;
        if let Some(v) = self.id_or_name.as_ref() {
            match v {
                id_or_name_value::IdOrName::Id(v) => {
                    struct_ser.serialize_field("id", v)?;
                }
                id_or_name_value::IdOrName::Name(v) => {
                    struct_ser.serialize_field("name", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdOrNameValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdOrNameValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.IdOrNameValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IdOrNameValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id_or_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id_or_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id_or_name__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| id_or_name_value::IdOrName::Id(x.0));
                        }
                        GeneratedField::Name => {
                            if id_or_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            id_or_name__ = map_.next_value::<::std::option::Option<_>>()?.map(id_or_name_value::IdOrName::Name);
                        }
                    }
                }
                Ok(IdOrNameValue {
                    id_or_name: id_or_name__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.IdOrNameValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportImageReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.accepted {
            len += 1;
        }
        if self.progress.is_some() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ImportImageReply", len)?;
        if self.accepted {
            struct_ser.serialize_field("accepted", &self.accepted)?;
        }
        if let Some(v) = self.progress.as_ref() {
            struct_ser.serialize_field("progress", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportImageReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "accepted",
            "progress",
            "spec",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Accepted,
            Progress,
            Spec,
            Metadata,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "accepted" => Ok(GeneratedField::Accepted),
                            "progress" => Ok(GeneratedField::Progress),
                            "spec" => Ok(GeneratedField::Spec),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportImageReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ImportImageReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportImageReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut accepted__ = None;
                let mut progress__ = None;
                let mut spec__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Accepted => {
                            if accepted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accepted"));
                            }
                            accepted__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Progress => {
                            if progress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("progress"));
                            }
                            progress__ = map_.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ImportImageReply {
                    accepted: accepted__.unwrap_or_default(),
                    progress: progress__,
                    spec: spec__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ImportImageReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportImageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.digest.is_empty() {
            len += 1;
        }
        if !self.image.is_empty() {
            len += 1;
        }
        if self.format != 0 {
            len += 1;
        }
        if !self.chunk.is_empty() {
            len += 1;
        }
        if self.last_chunk {
            len += 1;
        }
        if self.overwrite_cache {
            len += 1;
        }
        if self.want_metadata {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ImportImageRequest", len)?;
        if !self.digest.is_empty() {
            struct_ser.serialize_field("digest", &self.digest)?;
        }
        if !self.image.is_empty() {
            struct_ser.serialize_field("image", &self.image)?;
        }
        if self.format != 0 {
            let v = OciImageFormat::try_from(self.format)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.format)))?;
            struct_ser.serialize_field("format", &v)?;
        }
        if !self.chunk.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("chunk", pbjson::private::base64::encode(&self.chunk).as_str())?;
        }
        if self.last_chunk {
            struct_ser.serialize_field("lastChunk", &self.last_chunk)?;
        }
        if self.overwrite_cache {
            struct_ser.serialize_field("overwriteCache", &self.overwrite_cache)?;
        }
        if self.want_metadata {
            struct_ser.serialize_field("wantMetadata", &self.want_metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportImageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "digest",
            "image",
            "format",
            "chunk",
            "last_chunk",
            "lastChunk",
            "overwrite_cache",
            "overwriteCache",
            "want_metadata",
            "wantMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Digest,
            Image,
            Format,
            Chunk,
            LastChunk,
            OverwriteCache,
            WantMetadata,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "digest" => Ok(GeneratedField::Digest),
                            "image" => Ok(GeneratedField::Image),
                            "format" => Ok(GeneratedField::Format),
                            "chunk" => Ok(GeneratedField::Chunk),
                            "lastChunk" | "last_chunk" => Ok(GeneratedField::LastChunk),
                            "overwriteCache" | "overwrite_cache" => Ok(GeneratedField::OverwriteCache),
                            "wantMetadata" | "want_metadata" => Ok(GeneratedField::WantMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportImageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ImportImageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ImportImageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut digest__ = None;
                let mut image__ = None;
                let mut format__ = None;
                let mut chunk__ = None;
                let mut last_chunk__ = None;
                let mut overwrite_cache__ = None;
                let mut want_metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Digest => {
                            if digest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digest"));
                            }
                            digest__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Image => {
                            if image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            image__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Format => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format__ = Some(map_.next_value::<OciImageFormat>()? as i32);
                        }
                        GeneratedField::Chunk => {
                            if chunk__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunk"));
                            }
                            chunk__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LastChunk => {
                            if last_chunk__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastChunk"));
                            }
                            last_chunk__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OverwriteCache => {
                            if overwrite_cache__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overwriteCache"));
                            }
                            overwrite_cache__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WantMetadata => {
                            if want_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wantMetadata"));
                            }
                            want_metadata__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ImportImageRequest {
                    digest: digest__.unwrap_or_default(),
                    image: image__.unwrap_or_default(),
                    format: format__.unwrap_or_default(),
                    chunk: chunk__.unwrap_or_default(),
                    last_chunk: last_chunk__.unwrap_or_default(),
                    overwrite_cache: overwrite_cache__.unwrap_or_default(),
                    want_metadata: want_metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ImportImageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KernelModuleParameter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.module.is_empty() {
            len += 1;
        }
        if !self.parameter.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.KernelModuleParameter", len)?;
        if !self.module.is_empty() {
            struct_ser.serialize_field("module", &self.module)?;
        }
        if !self.parameter.is_empty() {
            struct_ser.serialize_field("parameter", &self.parameter)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KernelModuleParameter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "module",
            "parameter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Module,
            Parameter,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "module" => Ok(GeneratedField::Module),
                            "parameter" => Ok(GeneratedField::Parameter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KernelModuleParameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.KernelModuleParameter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KernelModuleParameter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut module__ = None;
                let mut parameter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Module => {
                            if module__.is_some() {
                                return Err(serde::de::Error::duplicate_field("module"));
                            }
                            module__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Parameter => {
                            if parameter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameter"));
                            }
                            parameter__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(KernelModuleParameter {
                    module: module__.unwrap_or_default(),
                    parameter: parameter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.KernelModuleParameter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KernelOptionsSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.verbose {
            len += 1;
        }
        if !self.cmdline_append.is_empty() {
            len += 1;
        }
        if !self.cmdline.is_empty() {
            len += 1;
        }
        if !self.modules.is_empty() {
            len += 1;
        }
        if !self.module_parameters.is_empty() {
            len += 1;
        }
        if !self.sysctl_parameters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.KernelOptionsSpec", len)?;
        if self.verbose {
            struct_ser.serialize_field("verbose", &self.verbose)?;
        }
        if !self.cmdline_append.is_empty() {
            struct_ser.serialize_field("cmdlineAppend", &self.cmdline_append)?;
        }
        if !self.cmdline.is_empty() {
            struct_ser.serialize_field("cmdline", &self.cmdline)?;
        }
        if !self.modules.is_empty() {
            struct_ser.serialize_field("modules", &self.modules)?;
        }
        if !self.module_parameters.is_empty() {
            struct_ser.serialize_field("moduleParameters", &self.module_parameters)?;
        }
        if !self.sysctl_parameters.is_empty() {
            struct_ser.serialize_field("sysctlParameters", &self.sysctl_parameters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KernelOptionsSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "verbose",
            "cmdline_append",
            "cmdlineAppend",
            "cmdline",
            "modules",
            "module_parameters",
            "moduleParameters",
            "sysctl_parameters",
            "sysctlParameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Verbose,
            CmdlineAppend,
            Cmdline,
            Modules,
            ModuleParameters,
            SysctlParameters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "verbose" => Ok(GeneratedField::Verbose),
                            "cmdlineAppend" | "cmdline_append" => Ok(GeneratedField::CmdlineAppend),
                            "cmdline" => Ok(GeneratedField::Cmdline),
                            "modules" => Ok(GeneratedField::Modules),
                            "moduleParameters" | "module_parameters" => Ok(GeneratedField::ModuleParameters),
                            "sysctlParameters" | "sysctl_parameters" => Ok(GeneratedField::SysctlParameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KernelOptionsSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.KernelOptionsSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KernelOptionsSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut verbose__ = None;
                let mut cmdline_append__ = None;
                let mut cmdline__ = None;
                let mut modules__ = None;
                let mut module_parameters__ = None;
                let mut sysctl_parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Verbose => {
                            if verbose__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verbose"));
                            }
                            verbose__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CmdlineAppend => {
                            if cmdline_append__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cmdlineAppend"));
                            }
                            cmdline_append__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cmdline => {
                            if cmdline__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cmdline"));
                            }
                            cmdline__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Modules => {
                            if modules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modules"));
                            }
                            modules__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModuleParameters => {
                            if module_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moduleParameters"));
                            }
                            module_parameters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SysctlParameters => {
                            if sysctl_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sysctlParameters"));
                            }
                            sysctl_parameters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(KernelOptionsSpec {
                    verbose: verbose__.unwrap_or_default(),
                    cmdline_append: cmdline_append__.unwrap_or_default(),
                    cmdline: cmdline__.unwrap_or_default(),
                    modules: modules__.unwrap_or_default(),
                    module_parameters: module_parameters__.unwrap_or_default(),
                    sysctl_parameters: sysctl_parameters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.KernelOptionsSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KernelSysctlParameter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.KernelSysctlParameter", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KernelSysctlParameter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KernelSysctlParameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.KernelSysctlParameter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<KernelSysctlParameter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(KernelSysctlParameter {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.KernelSysctlParameter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDevicesReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.devices.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ListDevicesReply", len)?;
        if !self.devices.is_empty() {
            struct_ser.serialize_field("devices", &self.devices)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDevicesReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "devices",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Devices,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "devices" => Ok(GeneratedField::Devices),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDevicesReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ListDevicesReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDevicesReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut devices__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Devices => {
                            if devices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("devices"));
                            }
                            devices__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListDevicesReply {
                    devices: devices__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ListDevicesReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListDevicesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.ListDevicesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListDevicesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListDevicesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ListDevicesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListDevicesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListDevicesRequest {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ListDevicesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListImagesReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.images.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ListImagesReply", len)?;
        if !self.images.is_empty() {
            struct_ser.serialize_field("images", &self.images)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListImagesReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "images",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Images,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "images" => Ok(GeneratedField::Images),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListImagesReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ListImagesReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListImagesReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut images__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Images => {
                            if images__.is_some() {
                                return Err(serde::de::Error::duplicate_field("images"));
                            }
                            images__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListImagesReply {
                    images: images__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ListImagesReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListImagesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.ListImagesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListImagesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListImagesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ListImagesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListImagesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListImagesRequest {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ListImagesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListNetworkReservationsReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reservations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ListNetworkReservationsReply", len)?;
        if !self.reservations.is_empty() {
            struct_ser.serialize_field("reservations", &self.reservations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListNetworkReservationsReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reservations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reservations,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "reservations" => Ok(GeneratedField::Reservations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListNetworkReservationsReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ListNetworkReservationsReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListNetworkReservationsReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reservations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reservations => {
                            if reservations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reservations"));
                            }
                            reservations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListNetworkReservationsReply {
                    reservations: reservations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ListNetworkReservationsReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListNetworkReservationsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.ListNetworkReservationsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListNetworkReservationsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListNetworkReservationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ListNetworkReservationsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListNetworkReservationsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListNetworkReservationsRequest {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ListNetworkReservationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWorkloadsReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.workloads.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ListWorkloadsReply", len)?;
        if !self.workloads.is_empty() {
            struct_ser.serialize_field("workloads", &self.workloads)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListWorkloadsReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workloads",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Workloads,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "workloads" => Ok(GeneratedField::Workloads),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListWorkloadsReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ListWorkloadsReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWorkloadsReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workloads__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Workloads => {
                            if workloads__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workloads"));
                            }
                            workloads__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListWorkloadsReply {
                    workloads: workloads__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ListWorkloadsReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListWorkloadsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.ListWorkloadsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListWorkloadsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListWorkloadsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ListWorkloadsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListWorkloadsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListWorkloadsRequest {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ListWorkloadsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListZonesReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zones.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ListZonesReply", len)?;
        if !self.zones.is_empty() {
            struct_ser.serialize_field("zones", &self.zones)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListZonesReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zones",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Zones,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zones" => Ok(GeneratedField::Zones),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListZonesReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ListZonesReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListZonesReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zones__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Zones => {
                            if zones__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zones"));
                            }
                            zones__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListZonesReply {
                    zones: zones__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ListZonesReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListZonesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.ListZonesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListZonesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListZonesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ListZonesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListZonesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListZonesRequest {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ListZonesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MetricFormat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "METRIC_FORMAT_UNKNOWN",
            Self::Bytes => "METRIC_FORMAT_BYTES",
            Self::Integer => "METRIC_FORMAT_INTEGER",
            Self::DurationSeconds => "METRIC_FORMAT_DURATION_SECONDS",
            Self::Percent => "METRIC_FORMAT_PERCENT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MetricFormat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "METRIC_FORMAT_UNKNOWN",
            "METRIC_FORMAT_BYTES",
            "METRIC_FORMAT_INTEGER",
            "METRIC_FORMAT_DURATION_SECONDS",
            "METRIC_FORMAT_PERCENT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetricFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "METRIC_FORMAT_UNKNOWN" => Ok(MetricFormat::Unknown),
                    "METRIC_FORMAT_BYTES" => Ok(MetricFormat::Bytes),
                    "METRIC_FORMAT_INTEGER" => Ok(MetricFormat::Integer),
                    "METRIC_FORMAT_DURATION_SECONDS" => Ok(MetricFormat::DurationSeconds),
                    "METRIC_FORMAT_PERCENT" => Ok(MetricFormat::Percent),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for MetricNode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if self.format != 0 {
            len += 1;
        }
        if !self.children.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.MetricNode", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if self.format != 0 {
            let v = MetricFormat::try_from(self.format)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.format)))?;
            struct_ser.serialize_field("format", &v)?;
        }
        if !self.children.is_empty() {
            struct_ser.serialize_field("children", &self.children)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MetricNode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "value",
            "format",
            "children",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Value,
            Format,
            Children,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "value" => Ok(GeneratedField::Value),
                            "format" => Ok(GeneratedField::Format),
                            "children" => Ok(GeneratedField::Children),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetricNode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.MetricNode")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MetricNode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut value__ = None;
                let mut format__ = None;
                let mut children__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map_.next_value()?;
                        }
                        GeneratedField::Format => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format__ = Some(map_.next_value::<MetricFormat>()? as i32);
                        }
                        GeneratedField::Children => {
                            if children__.is_some() {
                                return Err(serde::de::Error::duplicate_field("children"));
                            }
                            children__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MetricNode {
                    name: name__.unwrap_or_default(),
                    value: value__,
                    format: format__.unwrap_or_default(),
                    children: children__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.MetricNode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MonitorZoneKernelEventReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reply.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.MonitorZoneKernelEventReply", len)?;
        if let Some(v) = self.reply.as_ref() {
            match v {
                monitor_zone_kernel_event_reply::Reply::Syscall(v) => {
                    struct_ser.serialize_field("syscall", v)?;
                }
                monitor_zone_kernel_event_reply::Reply::Threadsnap(v) => {
                    struct_ser.serialize_field("threadsnap", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MonitorZoneKernelEventReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "syscall",
            "threadsnap",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Syscall,
            Threadsnap,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "syscall" => Ok(GeneratedField::Syscall),
                            "threadsnap" => Ok(GeneratedField::Threadsnap),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MonitorZoneKernelEventReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.MonitorZoneKernelEventReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MonitorZoneKernelEventReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reply__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Syscall => {
                            if reply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("syscall"));
                            }
                            reply__ = map_.next_value::<::std::option::Option<_>>()?.map(monitor_zone_kernel_event_reply::Reply::Syscall)
;
                        }
                        GeneratedField::Threadsnap => {
                            if reply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threadsnap"));
                            }
                            reply__ = map_.next_value::<::std::option::Option<_>>()?.map(monitor_zone_kernel_event_reply::Reply::Threadsnap)
;
                        }
                    }
                }
                Ok(MonitorZoneKernelEventReply {
                    reply: reply__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.MonitorZoneKernelEventReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MonitorZoneKernelEventRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_id.is_empty() {
            len += 1;
        }
        if self.request.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.MonitorZoneKernelEventRequest", len)?;
        if !self.zone_id.is_empty() {
            struct_ser.serialize_field("zoneId", &self.zone_id)?;
        }
        if let Some(v) = self.request.as_ref() {
            match v {
                monitor_zone_kernel_event_request::Request::Update(v) => {
                    struct_ser.serialize_field("update", v)?;
                }
                monitor_zone_kernel_event_request::Request::Stop(v) => {
                    struct_ser.serialize_field("stop", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MonitorZoneKernelEventRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_id",
            "zoneId",
            "update",
            "stop",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneId,
            Update,
            Stop,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            "update" => Ok(GeneratedField::Update),
                            "stop" => Ok(GeneratedField::Stop),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MonitorZoneKernelEventRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.MonitorZoneKernelEventRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MonitorZoneKernelEventRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_id__ = None;
                let mut request__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Update => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(monitor_zone_kernel_event_request::Request::Update)
;
                        }
                        GeneratedField::Stop => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stop"));
                            }
                            request__ = map_.next_value::<::std::option::Option<_>>()?.map(monitor_zone_kernel_event_request::Request::Stop)
;
                        }
                    }
                }
                Ok(MonitorZoneKernelEventRequest {
                    zone_id: zone_id__.unwrap_or_default(),
                    request: request__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.MonitorZoneKernelEventRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MountSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.host_path.is_empty() {
            len += 1;
        }
        if !self.target_path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.MountSpec", len)?;
        if !self.host_path.is_empty() {
            struct_ser.serialize_field("hostPath", &self.host_path)?;
        }
        if !self.target_path.is_empty() {
            struct_ser.serialize_field("targetPath", &self.target_path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MountSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host_path",
            "hostPath",
            "target_path",
            "targetPath",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostPath,
            TargetPath,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hostPath" | "host_path" => Ok(GeneratedField::HostPath),
                            "targetPath" | "target_path" => Ok(GeneratedField::TargetPath),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MountSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.MountSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MountSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host_path__ = None;
                let mut target_path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HostPath => {
                            if host_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostPath"));
                            }
                            host_path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TargetPath => {
                            if target_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetPath"));
                            }
                            target_path__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MountSpec {
                    host_path: host_path__.unwrap_or_default(),
                    target_path: target_path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.MountSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NetworkReservation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uuid.is_empty() {
            len += 1;
        }
        if !self.ipv4.is_empty() {
            len += 1;
        }
        if !self.ipv6.is_empty() {
            len += 1;
        }
        if !self.mac.is_empty() {
            len += 1;
        }
        if !self.gateway_ipv4.is_empty() {
            len += 1;
        }
        if !self.gateway_ipv6.is_empty() {
            len += 1;
        }
        if !self.gateway_mac.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.NetworkReservation", len)?;
        if !self.uuid.is_empty() {
            struct_ser.serialize_field("uuid", &self.uuid)?;
        }
        if !self.ipv4.is_empty() {
            struct_ser.serialize_field("ipv4", &self.ipv4)?;
        }
        if !self.ipv6.is_empty() {
            struct_ser.serialize_field("ipv6", &self.ipv6)?;
        }
        if !self.mac.is_empty() {
            struct_ser.serialize_field("mac", &self.mac)?;
        }
        if !self.gateway_ipv4.is_empty() {
            struct_ser.serialize_field("gatewayIpv4", &self.gateway_ipv4)?;
        }
        if !self.gateway_ipv6.is_empty() {
            struct_ser.serialize_field("gatewayIpv6", &self.gateway_ipv6)?;
        }
        if !self.gateway_mac.is_empty() {
            struct_ser.serialize_field("gatewayMac", &self.gateway_mac)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NetworkReservation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uuid",
            "ipv4",
            "ipv6",
            "mac",
            "gateway_ipv4",
            "gatewayIpv4",
            "gateway_ipv6",
            "gatewayIpv6",
            "gateway_mac",
            "gatewayMac",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uuid,
            Ipv4,
            Ipv6,
            Mac,
            GatewayIpv4,
            GatewayIpv6,
            GatewayMac,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "uuid" => Ok(GeneratedField::Uuid),
                            "ipv4" => Ok(GeneratedField::Ipv4),
                            "ipv6" => Ok(GeneratedField::Ipv6),
                            "mac" => Ok(GeneratedField::Mac),
                            "gatewayIpv4" | "gateway_ipv4" => Ok(GeneratedField::GatewayIpv4),
                            "gatewayIpv6" | "gateway_ipv6" => Ok(GeneratedField::GatewayIpv6),
                            "gatewayMac" | "gateway_mac" => Ok(GeneratedField::GatewayMac),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NetworkReservation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.NetworkReservation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NetworkReservation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uuid__ = None;
                let mut ipv4__ = None;
                let mut ipv6__ = None;
                let mut mac__ = None;
                let mut gateway_ipv4__ = None;
                let mut gateway_ipv6__ = None;
                let mut gateway_mac__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Uuid => {
                            if uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            uuid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ipv4 => {
                            if ipv4__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv4"));
                            }
                            ipv4__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ipv6 => {
                            if ipv6__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv6"));
                            }
                            ipv6__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Mac => {
                            if mac__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mac"));
                            }
                            mac__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GatewayIpv4 => {
                            if gateway_ipv4__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gatewayIpv4"));
                            }
                            gateway_ipv4__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GatewayIpv6 => {
                            if gateway_ipv6__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gatewayIpv6"));
                            }
                            gateway_ipv6__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GatewayMac => {
                            if gateway_mac__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gatewayMac"));
                            }
                            gateway_mac__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(NetworkReservation {
                    uuid: uuid__.unwrap_or_default(),
                    ipv4: ipv4__.unwrap_or_default(),
                    ipv6: ipv6__.unwrap_or_default(),
                    mac: mac__.unwrap_or_default(),
                    gateway_ipv4: gateway_ipv4__.unwrap_or_default(),
                    gateway_ipv6: gateway_ipv6__.unwrap_or_default(),
                    gateway_mac: gateway_mac__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.NetworkReservation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OciImageFormat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "OCI_IMAGE_FORMAT_UNKNOWN",
            Self::Squashfs => "OCI_IMAGE_FORMAT_SQUASHFS",
            Self::Erofs => "OCI_IMAGE_FORMAT_EROFS",
            Self::Tar => "OCI_IMAGE_FORMAT_TAR",
            Self::Directory => "OCI_IMAGE_FORMAT_DIRECTORY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OciImageFormat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OCI_IMAGE_FORMAT_UNKNOWN",
            "OCI_IMAGE_FORMAT_SQUASHFS",
            "OCI_IMAGE_FORMAT_EROFS",
            "OCI_IMAGE_FORMAT_TAR",
            "OCI_IMAGE_FORMAT_DIRECTORY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OciImageFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OCI_IMAGE_FORMAT_UNKNOWN" => Ok(OciImageFormat::Unknown),
                    "OCI_IMAGE_FORMAT_SQUASHFS" => Ok(OciImageFormat::Squashfs),
                    "OCI_IMAGE_FORMAT_EROFS" => Ok(OciImageFormat::Erofs),
                    "OCI_IMAGE_FORMAT_TAR" => Ok(OciImageFormat::Tar),
                    "OCI_IMAGE_FORMAT_DIRECTORY" => Ok(OciImageFormat::Directory),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OciImageInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.digest.is_empty() {
            len += 1;
        }
        if self.format != 0 {
            len += 1;
        }
        if !self.names.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.OciImageInfo", len)?;
        if !self.digest.is_empty() {
            struct_ser.serialize_field("digest", &self.digest)?;
        }
        if self.format != 0 {
            let v = OciImageFormat::try_from(self.format)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.format)))?;
            struct_ser.serialize_field("format", &v)?;
        }
        if !self.names.is_empty() {
            struct_ser.serialize_field("names", &self.names)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OciImageInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "digest",
            "format",
            "names",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Digest,
            Format,
            Names,
            Metadata,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "digest" => Ok(GeneratedField::Digest),
                            "format" => Ok(GeneratedField::Format),
                            "names" => Ok(GeneratedField::Names),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OciImageInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.OciImageInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OciImageInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut digest__ = None;
                let mut format__ = None;
                let mut names__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Digest => {
                            if digest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digest"));
                            }
                            digest__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Format => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format__ = Some(map_.next_value::<OciImageFormat>()? as i32);
                        }
                        GeneratedField::Names => {
                            if names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("names"));
                            }
                            names__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OciImageInfo {
                    digest: digest__.unwrap_or_default(),
                    format: format__.unwrap_or_default(),
                    names: names__.unwrap_or_default(),
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.OciImageInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OciImageMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.manifest.is_empty() {
            len += 1;
        }
        if !self.config.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.OciImageMetadata", len)?;
        if !self.manifest.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("manifest", pbjson::private::base64::encode(&self.manifest).as_str())?;
        }
        if !self.config.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("config", pbjson::private::base64::encode(&self.config).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OciImageMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "manifest",
            "config",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Manifest,
            Config,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "manifest" => Ok(GeneratedField::Manifest),
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OciImageMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.OciImageMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OciImageMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut manifest__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Manifest => {
                            if manifest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("manifest"));
                            }
                            manifest__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(OciImageMetadata {
                    manifest: manifest__.unwrap_or_default(),
                    config: config__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.OciImageMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OciImageProgress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.phase != 0 {
            len += 1;
        }
        if !self.layers.is_empty() {
            len += 1;
        }
        if self.indication.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.OciImageProgress", len)?;
        if self.phase != 0 {
            let v = OciImageProgressPhase::try_from(self.phase)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.phase)))?;
            struct_ser.serialize_field("phase", &v)?;
        }
        if !self.layers.is_empty() {
            struct_ser.serialize_field("layers", &self.layers)?;
        }
        if let Some(v) = self.indication.as_ref() {
            struct_ser.serialize_field("indication", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OciImageProgress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "phase",
            "layers",
            "indication",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Phase,
            Layers,
            Indication,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "phase" => Ok(GeneratedField::Phase),
                            "layers" => Ok(GeneratedField::Layers),
                            "indication" => Ok(GeneratedField::Indication),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OciImageProgress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.OciImageProgress")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OciImageProgress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut phase__ = None;
                let mut layers__ = None;
                let mut indication__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Phase => {
                            if phase__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phase"));
                            }
                            phase__ = Some(map_.next_value::<OciImageProgressPhase>()? as i32);
                        }
                        GeneratedField::Layers => {
                            if layers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("layers"));
                            }
                            layers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Indication => {
                            if indication__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indication"));
                            }
                            indication__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OciImageProgress {
                    phase: phase__.unwrap_or_default(),
                    layers: layers__.unwrap_or_default(),
                    indication: indication__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.OciImageProgress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OciImageProgressIndication {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.indication.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.OciImageProgressIndication", len)?;
        if let Some(v) = self.indication.as_ref() {
            match v {
                oci_image_progress_indication::Indication::Bar(v) => {
                    struct_ser.serialize_field("bar", v)?;
                }
                oci_image_progress_indication::Indication::Spinner(v) => {
                    struct_ser.serialize_field("spinner", v)?;
                }
                oci_image_progress_indication::Indication::Hidden(v) => {
                    struct_ser.serialize_field("hidden", v)?;
                }
                oci_image_progress_indication::Indication::Completed(v) => {
                    struct_ser.serialize_field("completed", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OciImageProgressIndication {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bar",
            "spinner",
            "hidden",
            "completed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bar,
            Spinner,
            Hidden,
            Completed,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "bar" => Ok(GeneratedField::Bar),
                            "spinner" => Ok(GeneratedField::Spinner),
                            "hidden" => Ok(GeneratedField::Hidden),
                            "completed" => Ok(GeneratedField::Completed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OciImageProgressIndication;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.OciImageProgressIndication")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OciImageProgressIndication, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut indication__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bar => {
                            if indication__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bar"));
                            }
                            indication__ = map_.next_value::<::std::option::Option<_>>()?.map(oci_image_progress_indication::Indication::Bar)
;
                        }
                        GeneratedField::Spinner => {
                            if indication__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spinner"));
                            }
                            indication__ = map_.next_value::<::std::option::Option<_>>()?.map(oci_image_progress_indication::Indication::Spinner)
;
                        }
                        GeneratedField::Hidden => {
                            if indication__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hidden"));
                            }
                            indication__ = map_.next_value::<::std::option::Option<_>>()?.map(oci_image_progress_indication::Indication::Hidden)
;
                        }
                        GeneratedField::Completed => {
                            if indication__.is_some() {
                                return Err(serde::de::Error::duplicate_field("completed"));
                            }
                            indication__ = map_.next_value::<::std::option::Option<_>>()?.map(oci_image_progress_indication::Indication::Completed)
;
                        }
                    }
                }
                Ok(OciImageProgressIndication {
                    indication: indication__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.OciImageProgressIndication", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OciImageProgressIndicationBar {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        if self.current != 0 {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        if self.is_bytes {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.OciImageProgressIndicationBar", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if self.current != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("current", ToString::to_string(&self.current).as_str())?;
        }
        if self.total != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total", ToString::to_string(&self.total).as_str())?;
        }
        if self.is_bytes {
            struct_ser.serialize_field("isBytes", &self.is_bytes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OciImageProgressIndicationBar {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
            "current",
            "total",
            "is_bytes",
            "isBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
            Current,
            Total,
            IsBytes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "message" => Ok(GeneratedField::Message),
                            "current" => Ok(GeneratedField::Current),
                            "total" => Ok(GeneratedField::Total),
                            "isBytes" | "is_bytes" => Ok(GeneratedField::IsBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OciImageProgressIndicationBar;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.OciImageProgressIndicationBar")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OciImageProgressIndicationBar, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                let mut current__ = None;
                let mut total__ = None;
                let mut is_bytes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Current => {
                            if current__.is_some() {
                                return Err(serde::de::Error::duplicate_field("current"));
                            }
                            current__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IsBytes => {
                            if is_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isBytes"));
                            }
                            is_bytes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OciImageProgressIndicationBar {
                    message: message__.unwrap_or_default(),
                    current: current__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                    is_bytes: is_bytes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.OciImageProgressIndicationBar", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OciImageProgressIndicationCompleted {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        if self.is_bytes {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.OciImageProgressIndicationCompleted", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if self.total != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("total", ToString::to_string(&self.total).as_str())?;
        }
        if self.is_bytes {
            struct_ser.serialize_field("isBytes", &self.is_bytes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OciImageProgressIndicationCompleted {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
            "total",
            "is_bytes",
            "isBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
            Total,
            IsBytes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "message" => Ok(GeneratedField::Message),
                            "total" => Ok(GeneratedField::Total),
                            "isBytes" | "is_bytes" => Ok(GeneratedField::IsBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OciImageProgressIndicationCompleted;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.OciImageProgressIndicationCompleted")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OciImageProgressIndicationCompleted, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                let mut total__ = None;
                let mut is_bytes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IsBytes => {
                            if is_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isBytes"));
                            }
                            is_bytes__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OciImageProgressIndicationCompleted {
                    message: message__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                    is_bytes: is_bytes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.OciImageProgressIndicationCompleted", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OciImageProgressIndicationHidden {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.OciImageProgressIndicationHidden", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OciImageProgressIndicationHidden {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OciImageProgressIndicationHidden;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.OciImageProgressIndicationHidden")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OciImageProgressIndicationHidden, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(OciImageProgressIndicationHidden {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.OciImageProgressIndicationHidden", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OciImageProgressIndicationSpinner {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.OciImageProgressIndicationSpinner", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OciImageProgressIndicationSpinner {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OciImageProgressIndicationSpinner;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.OciImageProgressIndicationSpinner")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OciImageProgressIndicationSpinner, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OciImageProgressIndicationSpinner {
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.OciImageProgressIndicationSpinner", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OciImageProgressLayer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.phase != 0 {
            len += 1;
        }
        if self.indication.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.OciImageProgressLayer", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.phase != 0 {
            let v = OciImageProgressLayerPhase::try_from(self.phase)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.phase)))?;
            struct_ser.serialize_field("phase", &v)?;
        }
        if let Some(v) = self.indication.as_ref() {
            struct_ser.serialize_field("indication", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OciImageProgressLayer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "phase",
            "indication",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Phase,
            Indication,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "phase" => Ok(GeneratedField::Phase),
                            "indication" => Ok(GeneratedField::Indication),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OciImageProgressLayer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.OciImageProgressLayer")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OciImageProgressLayer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut phase__ = None;
                let mut indication__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Phase => {
                            if phase__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phase"));
                            }
                            phase__ = Some(map_.next_value::<OciImageProgressLayerPhase>()? as i32);
                        }
                        GeneratedField::Indication => {
                            if indication__.is_some() {
                                return Err(serde::de::Error::duplicate_field("indication"));
                            }
                            indication__ = map_.next_value()?;
                        }
                    }
                }
                Ok(OciImageProgressLayer {
                    id: id__.unwrap_or_default(),
                    phase: phase__.unwrap_or_default(),
                    indication: indication__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.OciImageProgressLayer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OciImageProgressLayerPhase {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "OCI_IMAGE_PROGRESS_LAYER_PHASE_UNKNOWN",
            Self::Waiting => "OCI_IMAGE_PROGRESS_LAYER_PHASE_WAITING",
            Self::Downloading => "OCI_IMAGE_PROGRESS_LAYER_PHASE_DOWNLOADING",
            Self::Downloaded => "OCI_IMAGE_PROGRESS_LAYER_PHASE_DOWNLOADED",
            Self::Extracting => "OCI_IMAGE_PROGRESS_LAYER_PHASE_EXTRACTING",
            Self::Extracted => "OCI_IMAGE_PROGRESS_LAYER_PHASE_EXTRACTED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OciImageProgressLayerPhase {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OCI_IMAGE_PROGRESS_LAYER_PHASE_UNKNOWN",
            "OCI_IMAGE_PROGRESS_LAYER_PHASE_WAITING",
            "OCI_IMAGE_PROGRESS_LAYER_PHASE_DOWNLOADING",
            "OCI_IMAGE_PROGRESS_LAYER_PHASE_DOWNLOADED",
            "OCI_IMAGE_PROGRESS_LAYER_PHASE_EXTRACTING",
            "OCI_IMAGE_PROGRESS_LAYER_PHASE_EXTRACTED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OciImageProgressLayerPhase;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OCI_IMAGE_PROGRESS_LAYER_PHASE_UNKNOWN" => Ok(OciImageProgressLayerPhase::Unknown),
                    "OCI_IMAGE_PROGRESS_LAYER_PHASE_WAITING" => Ok(OciImageProgressLayerPhase::Waiting),
                    "OCI_IMAGE_PROGRESS_LAYER_PHASE_DOWNLOADING" => Ok(OciImageProgressLayerPhase::Downloading),
                    "OCI_IMAGE_PROGRESS_LAYER_PHASE_DOWNLOADED" => Ok(OciImageProgressLayerPhase::Downloaded),
                    "OCI_IMAGE_PROGRESS_LAYER_PHASE_EXTRACTING" => Ok(OciImageProgressLayerPhase::Extracting),
                    "OCI_IMAGE_PROGRESS_LAYER_PHASE_EXTRACTED" => Ok(OciImageProgressLayerPhase::Extracted),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OciImageProgressPhase {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "OCI_IMAGE_PROGRESS_PHASE_UNKNOWN",
            Self::Started => "OCI_IMAGE_PROGRESS_PHASE_STARTED",
            Self::Resolving => "OCI_IMAGE_PROGRESS_PHASE_RESOLVING",
            Self::Resolved => "OCI_IMAGE_PROGRESS_PHASE_RESOLVED",
            Self::ConfigDownload => "OCI_IMAGE_PROGRESS_PHASE_CONFIG_DOWNLOAD",
            Self::LayerDownload => "OCI_IMAGE_PROGRESS_PHASE_LAYER_DOWNLOAD",
            Self::Assemble => "OCI_IMAGE_PROGRESS_PHASE_ASSEMBLE",
            Self::Pack => "OCI_IMAGE_PROGRESS_PHASE_PACK",
            Self::Complete => "OCI_IMAGE_PROGRESS_PHASE_COMPLETE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OciImageProgressPhase {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OCI_IMAGE_PROGRESS_PHASE_UNKNOWN",
            "OCI_IMAGE_PROGRESS_PHASE_STARTED",
            "OCI_IMAGE_PROGRESS_PHASE_RESOLVING",
            "OCI_IMAGE_PROGRESS_PHASE_RESOLVED",
            "OCI_IMAGE_PROGRESS_PHASE_CONFIG_DOWNLOAD",
            "OCI_IMAGE_PROGRESS_PHASE_LAYER_DOWNLOAD",
            "OCI_IMAGE_PROGRESS_PHASE_ASSEMBLE",
            "OCI_IMAGE_PROGRESS_PHASE_PACK",
            "OCI_IMAGE_PROGRESS_PHASE_COMPLETE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OciImageProgressPhase;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OCI_IMAGE_PROGRESS_PHASE_UNKNOWN" => Ok(OciImageProgressPhase::Unknown),
                    "OCI_IMAGE_PROGRESS_PHASE_STARTED" => Ok(OciImageProgressPhase::Started),
                    "OCI_IMAGE_PROGRESS_PHASE_RESOLVING" => Ok(OciImageProgressPhase::Resolving),
                    "OCI_IMAGE_PROGRESS_PHASE_RESOLVED" => Ok(OciImageProgressPhase::Resolved),
                    "OCI_IMAGE_PROGRESS_PHASE_CONFIG_DOWNLOAD" => Ok(OciImageProgressPhase::ConfigDownload),
                    "OCI_IMAGE_PROGRESS_PHASE_LAYER_DOWNLOAD" => Ok(OciImageProgressPhase::LayerDownload),
                    "OCI_IMAGE_PROGRESS_PHASE_ASSEMBLE" => Ok(OciImageProgressPhase::Assemble),
                    "OCI_IMAGE_PROGRESS_PHASE_PACK" => Ok(OciImageProgressPhase::Pack),
                    "OCI_IMAGE_PROGRESS_PHASE_COMPLETE" => Ok(OciImageProgressPhase::Complete),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OciImageSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.digest.is_empty() {
            len += 1;
        }
        if self.format != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.OciImageSpec", len)?;
        if !self.digest.is_empty() {
            struct_ser.serialize_field("digest", &self.digest)?;
        }
        if self.format != 0 {
            let v = OciImageFormat::try_from(self.format)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.format)))?;
            struct_ser.serialize_field("format", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OciImageSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "digest",
            "format",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Digest,
            Format,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "digest" => Ok(GeneratedField::Digest),
                            "format" => Ok(GeneratedField::Format),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OciImageSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.OciImageSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OciImageSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut digest__ = None;
                let mut format__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Digest => {
                            if digest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digest"));
                            }
                            digest__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Format => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format__ = Some(map_.next_value::<OciImageFormat>()? as i32);
                        }
                    }
                }
                Ok(OciImageSpec {
                    digest: digest__.unwrap_or_default(),
                    format: format__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.OciImageSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OciRegistryAuthentication {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.authentication_method.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.OciRegistryAuthentication", len)?;
        if let Some(v) = self.authentication_method.as_ref() {
            match v {
                oci_registry_authentication::AuthenticationMethod::UsernamePassword(v) => {
                    struct_ser.serialize_field("usernamePassword", v)?;
                }
                oci_registry_authentication::AuthenticationMethod::RawBasicAuth(v) => {
                    struct_ser.serialize_field("rawBasicAuth", v)?;
                }
                oci_registry_authentication::AuthenticationMethod::IdentityToken(v) => {
                    struct_ser.serialize_field("identityToken", v)?;
                }
                oci_registry_authentication::AuthenticationMethod::RegistryToken(v) => {
                    struct_ser.serialize_field("registryToken", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OciRegistryAuthentication {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "username_password",
            "usernamePassword",
            "raw_basic_auth",
            "rawBasicAuth",
            "identity_token",
            "identityToken",
            "registry_token",
            "registryToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UsernamePassword,
            RawBasicAuth,
            IdentityToken,
            RegistryToken,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "usernamePassword" | "username_password" => Ok(GeneratedField::UsernamePassword),
                            "rawBasicAuth" | "raw_basic_auth" => Ok(GeneratedField::RawBasicAuth),
                            "identityToken" | "identity_token" => Ok(GeneratedField::IdentityToken),
                            "registryToken" | "registry_token" => Ok(GeneratedField::RegistryToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OciRegistryAuthentication;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.OciRegistryAuthentication")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OciRegistryAuthentication, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authentication_method__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UsernamePassword => {
                            if authentication_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usernamePassword"));
                            }
                            authentication_method__ = map_.next_value::<::std::option::Option<_>>()?.map(oci_registry_authentication::AuthenticationMethod::UsernamePassword)
;
                        }
                        GeneratedField::RawBasicAuth => {
                            if authentication_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rawBasicAuth"));
                            }
                            authentication_method__ = map_.next_value::<::std::option::Option<_>>()?.map(oci_registry_authentication::AuthenticationMethod::RawBasicAuth);
                        }
                        GeneratedField::IdentityToken => {
                            if authentication_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identityToken"));
                            }
                            authentication_method__ = map_.next_value::<::std::option::Option<_>>()?.map(oci_registry_authentication::AuthenticationMethod::IdentityToken);
                        }
                        GeneratedField::RegistryToken => {
                            if authentication_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registryToken"));
                            }
                            authentication_method__ = map_.next_value::<::std::option::Option<_>>()?.map(oci_registry_authentication::AuthenticationMethod::RegistryToken);
                        }
                    }
                }
                Ok(OciRegistryAuthentication {
                    authentication_method: authentication_method__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.OciRegistryAuthentication", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OciRegistryUsernamePassword {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.password.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.OciRegistryUsernamePassword", len)?;
        if !self.username.is_empty() {
            struct_ser.serialize_field("username", &self.username)?;
        }
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OciRegistryUsernamePassword {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "username",
            "password",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Username,
            Password,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "username" => Ok(GeneratedField::Username),
                            "password" => Ok(GeneratedField::Password),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OciRegistryUsernamePassword;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.OciRegistryUsernamePassword")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OciRegistryUsernamePassword, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut username__ = None;
                let mut password__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OciRegistryUsernamePassword {
                    username: username__.unwrap_or_default(),
                    password: password__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.OciRegistryUsernamePassword", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProcessNamespace {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Mount => "PROCESS_NAMESPACE_MOUNT",
            Self::Uts => "PROCESS_NAMESPACE_UTS",
            Self::Ipc => "PROCESS_NAMESPACE_IPC",
            Self::User => "PROCESS_NAMESPACE_USER",
            Self::Pid => "PROCESS_NAMESPACE_PID",
            Self::Net => "PROCESS_NAMESPACE_NET",
            Self::Cgroup => "PROCESS_NAMESPACE_CGROUP",
            Self::Time => "PROCESS_NAMESPACE_TIME",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ProcessNamespace {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PROCESS_NAMESPACE_MOUNT",
            "PROCESS_NAMESPACE_UTS",
            "PROCESS_NAMESPACE_IPC",
            "PROCESS_NAMESPACE_USER",
            "PROCESS_NAMESPACE_PID",
            "PROCESS_NAMESPACE_NET",
            "PROCESS_NAMESPACE_CGROUP",
            "PROCESS_NAMESPACE_TIME",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProcessNamespace;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PROCESS_NAMESPACE_MOUNT" => Ok(ProcessNamespace::Mount),
                    "PROCESS_NAMESPACE_UTS" => Ok(ProcessNamespace::Uts),
                    "PROCESS_NAMESPACE_IPC" => Ok(ProcessNamespace::Ipc),
                    "PROCESS_NAMESPACE_USER" => Ok(ProcessNamespace::User),
                    "PROCESS_NAMESPACE_PID" => Ok(ProcessNamespace::Pid),
                    "PROCESS_NAMESPACE_NET" => Ok(ProcessNamespace::Net),
                    "PROCESS_NAMESPACE_CGROUP" => Ok(ProcessNamespace::Cgroup),
                    "PROCESS_NAMESPACE_TIME" => Ok(ProcessNamespace::Time),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ProcessSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.environment.is_empty() {
            len += 1;
        }
        if !self.command.is_empty() {
            len += 1;
        }
        if !self.working_directory.is_empty() {
            len += 1;
        }
        if self.tty {
            len += 1;
        }
        if self.terminal_size.is_some() {
            len += 1;
        }
        if self.user.is_some() {
            len += 1;
        }
        if self.group.is_some() {
            len += 1;
        }
        if self.stdin {
            len += 1;
        }
        if self.stdout {
            len += 1;
        }
        if self.stderr {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ProcessSpec", len)?;
        if !self.environment.is_empty() {
            struct_ser.serialize_field("environment", &self.environment)?;
        }
        if !self.command.is_empty() {
            struct_ser.serialize_field("command", &self.command)?;
        }
        if !self.working_directory.is_empty() {
            struct_ser.serialize_field("workingDirectory", &self.working_directory)?;
        }
        if self.tty {
            struct_ser.serialize_field("tty", &self.tty)?;
        }
        if let Some(v) = self.terminal_size.as_ref() {
            struct_ser.serialize_field("terminalSize", v)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        if let Some(v) = self.group.as_ref() {
            struct_ser.serialize_field("group", v)?;
        }
        if self.stdin {
            struct_ser.serialize_field("stdin", &self.stdin)?;
        }
        if self.stdout {
            struct_ser.serialize_field("stdout", &self.stdout)?;
        }
        if self.stderr {
            struct_ser.serialize_field("stderr", &self.stderr)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProcessSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "environment",
            "command",
            "working_directory",
            "workingDirectory",
            "tty",
            "terminal_size",
            "terminalSize",
            "user",
            "group",
            "stdin",
            "stdout",
            "stderr",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Environment,
            Command,
            WorkingDirectory,
            Tty,
            TerminalSize,
            User,
            Group,
            Stdin,
            Stdout,
            Stderr,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "environment" => Ok(GeneratedField::Environment),
                            "command" => Ok(GeneratedField::Command),
                            "workingDirectory" | "working_directory" => Ok(GeneratedField::WorkingDirectory),
                            "tty" => Ok(GeneratedField::Tty),
                            "terminalSize" | "terminal_size" => Ok(GeneratedField::TerminalSize),
                            "user" => Ok(GeneratedField::User),
                            "group" => Ok(GeneratedField::Group),
                            "stdin" => Ok(GeneratedField::Stdin),
                            "stdout" => Ok(GeneratedField::Stdout),
                            "stderr" => Ok(GeneratedField::Stderr),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProcessSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ProcessSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ProcessSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut environment__ = None;
                let mut command__ = None;
                let mut working_directory__ = None;
                let mut tty__ = None;
                let mut terminal_size__ = None;
                let mut user__ = None;
                let mut group__ = None;
                let mut stdin__ = None;
                let mut stdout__ = None;
                let mut stderr__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Environment => {
                            if environment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("environment"));
                            }
                            environment__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Command => {
                            if command__.is_some() {
                                return Err(serde::de::Error::duplicate_field("command"));
                            }
                            command__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WorkingDirectory => {
                            if working_directory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workingDirectory"));
                            }
                            working_directory__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Tty => {
                            if tty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tty"));
                            }
                            tty__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TerminalSize => {
                            if terminal_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("terminalSize"));
                            }
                            terminal_size__ = map_.next_value()?;
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = map_.next_value()?;
                        }
                        GeneratedField::Group => {
                            if group__.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            group__ = map_.next_value()?;
                        }
                        GeneratedField::Stdin => {
                            if stdin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdin"));
                            }
                            stdin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Stdout => {
                            if stdout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdout"));
                            }
                            stdout__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Stderr => {
                            if stderr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stderr"));
                            }
                            stderr__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ProcessSpec {
                    environment: environment__.unwrap_or_default(),
                    command: command__.unwrap_or_default(),
                    working_directory: working_directory__.unwrap_or_default(),
                    tty: tty__.unwrap_or_default(),
                    terminal_size: terminal_size__,
                    user: user__,
                    group: group__,
                    stdin: stdin__.unwrap_or_default(),
                    stdout: stdout__.unwrap_or_default(),
                    stderr: stderr__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ProcessSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PullImageReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.progress.is_some() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.PullImageReply", len)?;
        if let Some(v) = self.progress.as_ref() {
            struct_ser.serialize_field("progress", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PullImageReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "progress",
            "spec",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Progress,
            Spec,
            Metadata,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "progress" => Ok(GeneratedField::Progress),
                            "spec" => Ok(GeneratedField::Spec),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PullImageReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.PullImageReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PullImageReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut progress__ = None;
                let mut spec__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Progress => {
                            if progress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("progress"));
                            }
                            progress__ = map_.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PullImageReply {
                    progress: progress__,
                    spec: spec__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.PullImageReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PullImageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.image.is_empty() {
            len += 1;
        }
        if self.format != 0 {
            len += 1;
        }
        if self.overwrite_cache {
            len += 1;
        }
        if self.update {
            len += 1;
        }
        if self.want_metadata {
            len += 1;
        }
        if self.auth.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.PullImageRequest", len)?;
        if !self.image.is_empty() {
            struct_ser.serialize_field("image", &self.image)?;
        }
        if self.format != 0 {
            let v = OciImageFormat::try_from(self.format)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.format)))?;
            struct_ser.serialize_field("format", &v)?;
        }
        if self.overwrite_cache {
            struct_ser.serialize_field("overwriteCache", &self.overwrite_cache)?;
        }
        if self.update {
            struct_ser.serialize_field("update", &self.update)?;
        }
        if self.want_metadata {
            struct_ser.serialize_field("wantMetadata", &self.want_metadata)?;
        }
        if let Some(v) = self.auth.as_ref() {
            struct_ser.serialize_field("auth", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PullImageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "image",
            "format",
            "overwrite_cache",
            "overwriteCache",
            "update",
            "want_metadata",
            "wantMetadata",
            "auth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Image,
            Format,
            OverwriteCache,
            Update,
            WantMetadata,
            Auth,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "image" => Ok(GeneratedField::Image),
                            "format" => Ok(GeneratedField::Format),
                            "overwriteCache" | "overwrite_cache" => Ok(GeneratedField::OverwriteCache),
                            "update" => Ok(GeneratedField::Update),
                            "wantMetadata" | "want_metadata" => Ok(GeneratedField::WantMetadata),
                            "auth" => Ok(GeneratedField::Auth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PullImageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.PullImageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PullImageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut image__ = None;
                let mut format__ = None;
                let mut overwrite_cache__ = None;
                let mut update__ = None;
                let mut want_metadata__ = None;
                let mut auth__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Image => {
                            if image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            image__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Format => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format__ = Some(map_.next_value::<OciImageFormat>()? as i32);
                        }
                        GeneratedField::OverwriteCache => {
                            if overwrite_cache__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overwriteCache"));
                            }
                            overwrite_cache__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Update => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update"));
                            }
                            update__ = Some(map_.next_value()?);
                        }
                        GeneratedField::WantMetadata => {
                            if want_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wantMetadata"));
                            }
                            want_metadata__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Auth => {
                            if auth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("auth"));
                            }
                            auth__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PullImageRequest {
                    image: image__.unwrap_or_default(),
                    format: format__.unwrap_or_default(),
                    overwrite_cache: overwrite_cache__.unwrap_or_default(),
                    update: update__.unwrap_or_default(),
                    want_metadata: want_metadata__.unwrap_or_default(),
                    auth: auth__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.PullImageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadHypervisorConsoleReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ReadHypervisorConsoleReply", len)?;
        if !self.data.is_empty() {
            struct_ser.serialize_field("data", &self.data)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadHypervisorConsoleReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadHypervisorConsoleReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ReadHypervisorConsoleReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadHypervisorConsoleReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadHypervisorConsoleReply {
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ReadHypervisorConsoleReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadHypervisorConsoleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.ReadHypervisorConsoleRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadHypervisorConsoleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadHypervisorConsoleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ReadHypervisorConsoleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadHypervisorConsoleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ReadHypervisorConsoleRequest {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ReadHypervisorConsoleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadZoneMetricsReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.root.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ReadZoneMetricsReply", len)?;
        if let Some(v) = self.root.as_ref() {
            struct_ser.serialize_field("root", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadZoneMetricsReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "root",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Root,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "root" => Ok(GeneratedField::Root),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadZoneMetricsReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ReadZoneMetricsReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadZoneMetricsReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut root__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Root => {
                            if root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("root"));
                            }
                            root__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ReadZoneMetricsReply {
                    root: root__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ReadZoneMetricsReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadZoneMetricsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ReadZoneMetricsRequest", len)?;
        if !self.zone_id.is_empty() {
            struct_ser.serialize_field("zoneId", &self.zone_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadZoneMetricsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_id",
            "zoneId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadZoneMetricsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ReadZoneMetricsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadZoneMetricsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ReadZoneMetricsRequest {
                    zone_id: zone_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ReadZoneMetricsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveImageReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.removed {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.RemoveImageReply", len)?;
        if self.removed {
            struct_ser.serialize_field("removed", &self.removed)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveImageReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "removed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Removed,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "removed" => Ok(GeneratedField::Removed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveImageReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.RemoveImageReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveImageReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut removed__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Removed => {
                            if removed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removed"));
                            }
                            removed__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RemoveImageReply {
                    removed: removed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.RemoveImageReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoveImageRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.digest.is_empty() {
            len += 1;
        }
        if self.format != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.RemoveImageRequest", len)?;
        if !self.digest.is_empty() {
            struct_ser.serialize_field("digest", &self.digest)?;
        }
        if self.format != 0 {
            let v = OciImageFormat::try_from(self.format)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.format)))?;
            struct_ser.serialize_field("format", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoveImageRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "digest",
            "format",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Digest,
            Format,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "digest" => Ok(GeneratedField::Digest),
                            "format" => Ok(GeneratedField::Format),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoveImageRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.RemoveImageRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoveImageRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut digest__ = None;
                let mut format__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Digest => {
                            if digest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("digest"));
                            }
                            digest__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Format => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format__ = Some(map_.next_value::<OciImageFormat>()? as i32);
                        }
                    }
                }
                Ok(RemoveImageRequest {
                    digest: digest__.unwrap_or_default(),
                    format: format__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.RemoveImageRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolveWorkloadIdReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.workload_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ResolveWorkloadIdReply", len)?;
        if !self.workload_id.is_empty() {
            struct_ser.serialize_field("workloadId", &self.workload_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolveWorkloadIdReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workload_id",
            "workloadId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkloadId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "workloadId" | "workload_id" => Ok(GeneratedField::WorkloadId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolveWorkloadIdReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ResolveWorkloadIdReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolveWorkloadIdReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workload_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkloadId => {
                            if workload_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workloadId"));
                            }
                            workload_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResolveWorkloadIdReply {
                    workload_id: workload_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ResolveWorkloadIdReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolveWorkloadIdRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ResolveWorkloadIdRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolveWorkloadIdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolveWorkloadIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ResolveWorkloadIdRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolveWorkloadIdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResolveWorkloadIdRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ResolveWorkloadIdRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolveWorkloadIdsReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.workload_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ResolveWorkloadIdsReply", len)?;
        if !self.workload_ids.is_empty() {
            struct_ser.serialize_field("workloadIds", &self.workload_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolveWorkloadIdsReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workload_ids",
            "workloadIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkloadIds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "workloadIds" | "workload_ids" => Ok(GeneratedField::WorkloadIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolveWorkloadIdsReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ResolveWorkloadIdsReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolveWorkloadIdsReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workload_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkloadIds => {
                            if workload_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workloadIds"));
                            }
                            workload_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResolveWorkloadIdsReply {
                    workload_ids: workload_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ResolveWorkloadIdsReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolveWorkloadIdsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ResolveWorkloadIdsRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolveWorkloadIdsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolveWorkloadIdsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ResolveWorkloadIdsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolveWorkloadIdsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResolveWorkloadIdsRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ResolveWorkloadIdsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolveZoneIdReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ResolveZoneIdReply", len)?;
        if !self.zone_id.is_empty() {
            struct_ser.serialize_field("zoneId", &self.zone_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolveZoneIdReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_id",
            "zoneId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolveZoneIdReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ResolveZoneIdReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolveZoneIdReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResolveZoneIdReply {
                    zone_id: zone_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ResolveZoneIdReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolveZoneIdRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ResolveZoneIdRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolveZoneIdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolveZoneIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ResolveZoneIdRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolveZoneIdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResolveZoneIdRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ResolveZoneIdRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolveZoneIdsReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ResolveZoneIdsReply", len)?;
        if !self.zone_ids.is_empty() {
            struct_ser.serialize_field("zoneIds", &self.zone_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolveZoneIdsReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_ids",
            "zoneIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneIds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneIds" | "zone_ids" => Ok(GeneratedField::ZoneIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolveZoneIdsReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ResolveZoneIdsReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolveZoneIdsReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneIds => {
                            if zone_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneIds"));
                            }
                            zone_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResolveZoneIdsReply {
                    zone_ids: zone_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ResolveZoneIdsReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResolveZoneIdsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ResolveZoneIdsRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResolveZoneIdsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResolveZoneIdsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ResolveZoneIdsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResolveZoneIdsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResolveZoneIdsRequest {
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ResolveZoneIdsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetHostPowerManagementPolicyReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.SetHostPowerManagementPolicyReply", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetHostPowerManagementPolicyReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetHostPowerManagementPolicyReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.SetHostPowerManagementPolicyReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetHostPowerManagementPolicyReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SetHostPowerManagementPolicyReply {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.SetHostPowerManagementPolicyReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetHostPowerManagementPolicyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.scheduler.is_empty() {
            len += 1;
        }
        if self.smt_awareness {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.SetHostPowerManagementPolicyRequest", len)?;
        if !self.scheduler.is_empty() {
            struct_ser.serialize_field("scheduler", &self.scheduler)?;
        }
        if self.smt_awareness {
            struct_ser.serialize_field("smtAwareness", &self.smt_awareness)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetHostPowerManagementPolicyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "scheduler",
            "smt_awareness",
            "smtAwareness",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Scheduler,
            SmtAwareness,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "scheduler" => Ok(GeneratedField::Scheduler),
                            "smtAwareness" | "smt_awareness" => Ok(GeneratedField::SmtAwareness),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetHostPowerManagementPolicyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.SetHostPowerManagementPolicyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SetHostPowerManagementPolicyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut scheduler__ = None;
                let mut smt_awareness__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Scheduler => {
                            if scheduler__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scheduler"));
                            }
                            scheduler__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SmtAwareness => {
                            if smt_awareness__.is_some() {
                                return Err(serde::de::Error::duplicate_field("smtAwareness"));
                            }
                            smt_awareness__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SetHostPowerManagementPolicyRequest {
                    scheduler: scheduler__.unwrap_or_default(),
                    smt_awareness: smt_awareness__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.SetHostPowerManagementPolicyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SnoopIdmPacket {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.to.is_empty() {
            len += 1;
        }
        if !self.packet.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.SnoopIdmPacket", len)?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.to.is_empty() {
            struct_ser.serialize_field("to", &self.to)?;
        }
        if !self.packet.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("packet", pbjson::private::base64::encode(&self.packet).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SnoopIdmPacket {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "from",
            "to",
            "packet",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            To,
            Packet,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "from" => Ok(GeneratedField::From),
                            "to" => Ok(GeneratedField::To),
                            "packet" => Ok(GeneratedField::Packet),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SnoopIdmPacket;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.SnoopIdmPacket")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SnoopIdmPacket, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut to__ = None;
                let mut packet__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map_.next_value()?);
                        }
                        GeneratedField::To => {
                            if to__.is_some() {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            to__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Packet => {
                            if packet__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packet"));
                            }
                            packet__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SnoopIdmPacket {
                    from: from__.unwrap_or_default(),
                    to: to__.unwrap_or_default(),
                    packet: packet__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.SnoopIdmPacket", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SnoopIdmReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.packets.is_empty() {
            len += 1;
        }
        if self.skipped != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.SnoopIdmReply", len)?;
        if !self.packets.is_empty() {
            struct_ser.serialize_field("packets", &self.packets)?;
        }
        if self.skipped != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("skipped", ToString::to_string(&self.skipped).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SnoopIdmReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "packets",
            "skipped",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Packets,
            Skipped,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "packets" => Ok(GeneratedField::Packets),
                            "skipped" => Ok(GeneratedField::Skipped),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SnoopIdmReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.SnoopIdmReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SnoopIdmReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut packets__ = None;
                let mut skipped__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Packets => {
                            if packets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packets"));
                            }
                            packets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Skipped => {
                            if skipped__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipped"));
                            }
                            skipped__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SnoopIdmReply {
                    packets: packets__.unwrap_or_default(),
                    skipped: skipped__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.SnoopIdmReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SnoopIdmRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.SnoopIdmRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SnoopIdmRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SnoopIdmRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.SnoopIdmRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SnoopIdmRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SnoopIdmRequest {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.SnoopIdmRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StartWorkloadReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.StartWorkloadReply", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StartWorkloadReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StartWorkloadReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.StartWorkloadReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StartWorkloadReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(StartWorkloadReply {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.StartWorkloadReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StartWorkloadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.workload_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.StartWorkloadRequest", len)?;
        if !self.workload_id.is_empty() {
            struct_ser.serialize_field("workloadId", &self.workload_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StartWorkloadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workload_id",
            "workloadId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkloadId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "workloadId" | "workload_id" => Ok(GeneratedField::WorkloadId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StartWorkloadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.StartWorkloadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StartWorkloadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workload_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkloadId => {
                            if workload_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workloadId"));
                            }
                            workload_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StartWorkloadRequest {
                    workload_id: workload_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.StartWorkloadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StopWorkloadReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.StopWorkloadReply", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StopWorkloadReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StopWorkloadReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.StopWorkloadReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StopWorkloadReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(StopWorkloadReply {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.StopWorkloadReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StopWorkloadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.workload_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.StopWorkloadRequest", len)?;
        if !self.workload_id.is_empty() {
            struct_ser.serialize_field("workloadId", &self.workload_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StopWorkloadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workload_id",
            "workloadId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkloadId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "workloadId" | "workload_id" => Ok(GeneratedField::WorkloadId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StopWorkloadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.StopWorkloadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StopWorkloadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workload_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkloadId => {
                            if workload_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workloadId"));
                            }
                            workload_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(StopWorkloadRequest {
                    workload_id: workload_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.StopWorkloadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TerminalSize {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rows != 0 {
            len += 1;
        }
        if self.columns != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.TerminalSize", len)?;
        if self.rows != 0 {
            struct_ser.serialize_field("rows", &self.rows)?;
        }
        if self.columns != 0 {
            struct_ser.serialize_field("columns", &self.columns)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TerminalSize {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rows",
            "columns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rows,
            Columns,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rows" => Ok(GeneratedField::Rows),
                            "columns" => Ok(GeneratedField::Columns),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TerminalSize;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.TerminalSize")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TerminalSize, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rows__ = None;
                let mut columns__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rows => {
                            if rows__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rows"));
                            }
                            rows__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Columns => {
                            if columns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("columns"));
                            }
                            columns__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TerminalSize {
                    rows: rows__.unwrap_or_default(),
                    columns: columns__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.TerminalSize", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateZoneResourcesReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.UpdateZoneResourcesReply", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateZoneResourcesReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateZoneResourcesReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.UpdateZoneResourcesReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateZoneResourcesReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(UpdateZoneResourcesReply {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.UpdateZoneResourcesReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateZoneResourcesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_id.is_empty() {
            len += 1;
        }
        if self.resources.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.UpdateZoneResourcesRequest", len)?;
        if !self.zone_id.is_empty() {
            struct_ser.serialize_field("zoneId", &self.zone_id)?;
        }
        if let Some(v) = self.resources.as_ref() {
            struct_ser.serialize_field("resources", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateZoneResourcesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_id",
            "zoneId",
            "resources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneId,
            Resources,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            "resources" => Ok(GeneratedField::Resources),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateZoneResourcesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.UpdateZoneResourcesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateZoneResourcesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_id__ = None;
                let mut resources__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateZoneResourcesRequest {
                    zone_id: zone_id__.unwrap_or_default(),
                    resources: resources__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.UpdateZoneResourcesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WatchEventsReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.WatchEventsReply", len)?;
        if let Some(v) = self.event.as_ref() {
            match v {
                watch_events_reply::Event::ZoneChanged(v) => {
                    struct_ser.serialize_field("zoneChanged", v)?;
                }
                watch_events_reply::Event::WorkloadChanged(v) => {
                    struct_ser.serialize_field("workloadChanged", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WatchEventsReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_changed",
            "zoneChanged",
            "workload_changed",
            "workloadChanged",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneChanged,
            WorkloadChanged,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneChanged" | "zone_changed" => Ok(GeneratedField::ZoneChanged),
                            "workloadChanged" | "workload_changed" => Ok(GeneratedField::WorkloadChanged),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WatchEventsReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.WatchEventsReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WatchEventsReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneChanged => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneChanged"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(watch_events_reply::Event::ZoneChanged)
;
                        }
                        GeneratedField::WorkloadChanged => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workloadChanged"));
                            }
                            event__ = map_.next_value::<::std::option::Option<_>>()?.map(watch_events_reply::Event::WorkloadChanged)
;
                        }
                    }
                }
                Ok(WatchEventsReply {
                    event: event__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.WatchEventsReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WatchEventsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.WatchEventsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WatchEventsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WatchEventsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.WatchEventsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WatchEventsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(WatchEventsRequest {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.WatchEventsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Workload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.Workload", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Workload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Spec,
            Status,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Workload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.Workload")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Workload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Workload {
                    id: id__.unwrap_or_default(),
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.Workload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkloadBlockDeviceInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.block_index != 0 {
            len += 1;
        }
        if self.device_id != 0 {
            len += 1;
        }
        if !self.loop_device.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.WorkloadBlockDeviceInfo", len)?;
        if self.block_index != 0 {
            struct_ser.serialize_field("blockIndex", &self.block_index)?;
        }
        if self.device_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("deviceId", ToString::to_string(&self.device_id).as_str())?;
        }
        if !self.loop_device.is_empty() {
            struct_ser.serialize_field("loopDevice", &self.loop_device)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkloadBlockDeviceInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block_index",
            "blockIndex",
            "device_id",
            "deviceId",
            "loop_device",
            "loopDevice",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockIndex,
            DeviceId,
            LoopDevice,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "blockIndex" | "block_index" => Ok(GeneratedField::BlockIndex),
                            "deviceId" | "device_id" => Ok(GeneratedField::DeviceId),
                            "loopDevice" | "loop_device" => Ok(GeneratedField::LoopDevice),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkloadBlockDeviceInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.WorkloadBlockDeviceInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkloadBlockDeviceInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block_index__ = None;
                let mut device_id__ = None;
                let mut loop_device__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BlockIndex => {
                            if block_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockIndex"));
                            }
                            block_index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DeviceId => {
                            if device_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceId"));
                            }
                            device_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LoopDevice => {
                            if loop_device__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loopDevice"));
                            }
                            loop_device__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WorkloadBlockDeviceInfo {
                    block_index: block_index__.unwrap_or_default(),
                    device_id: device_id__.unwrap_or_default(),
                    loop_device: loop_device__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.WorkloadBlockDeviceInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkloadBlockDeviceStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.devices.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.WorkloadBlockDeviceStatus", len)?;
        if !self.devices.is_empty() {
            struct_ser.serialize_field("devices", &self.devices)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkloadBlockDeviceStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "devices",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Devices,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "devices" => Ok(GeneratedField::Devices),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkloadBlockDeviceStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.WorkloadBlockDeviceStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkloadBlockDeviceStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut devices__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Devices => {
                            if devices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("devices"));
                            }
                            devices__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WorkloadBlockDeviceStatus {
                    devices: devices__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.WorkloadBlockDeviceStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkloadChangedEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.workload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.WorkloadChangedEvent", len)?;
        if let Some(v) = self.workload.as_ref() {
            struct_ser.serialize_field("workload", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkloadChangedEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workload",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Workload,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "workload" => Ok(GeneratedField::Workload),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkloadChangedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.WorkloadChangedEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkloadChangedEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Workload => {
                            if workload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workload"));
                            }
                            workload__ = map_.next_value()?;
                        }
                    }
                }
                Ok(WorkloadChangedEvent {
                    workload: workload__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.WorkloadChangedEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkloadConsoleReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stdout.is_empty() {
            len += 1;
        }
        if !self.stderr.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.WorkloadConsoleReply", len)?;
        if !self.stdout.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("stdout", pbjson::private::base64::encode(&self.stdout).as_str())?;
        }
        if !self.stderr.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("stderr", pbjson::private::base64::encode(&self.stderr).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkloadConsoleReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stdout",
            "stderr",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Stdout,
            Stderr,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stdout" => Ok(GeneratedField::Stdout),
                            "stderr" => Ok(GeneratedField::Stderr),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkloadConsoleReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.WorkloadConsoleReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkloadConsoleReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stdout__ = None;
                let mut stderr__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Stdout => {
                            if stdout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdout"));
                            }
                            stdout__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Stderr => {
                            if stderr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stderr"));
                            }
                            stderr__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(WorkloadConsoleReply {
                    stdout: stdout__.unwrap_or_default(),
                    stderr: stderr__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.WorkloadConsoleReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkloadConsoleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.workload_id.is_empty() {
            len += 1;
        }
        if !self.stdin.is_empty() {
            len += 1;
        }
        if self.stdin_closed {
            len += 1;
        }
        if self.terminal_resize.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.WorkloadConsoleRequest", len)?;
        if !self.workload_id.is_empty() {
            struct_ser.serialize_field("workloadId", &self.workload_id)?;
        }
        if !self.stdin.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("stdin", pbjson::private::base64::encode(&self.stdin).as_str())?;
        }
        if self.stdin_closed {
            struct_ser.serialize_field("stdinClosed", &self.stdin_closed)?;
        }
        if let Some(v) = self.terminal_resize.as_ref() {
            struct_ser.serialize_field("terminalResize", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkloadConsoleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "workload_id",
            "workloadId",
            "stdin",
            "stdin_closed",
            "stdinClosed",
            "terminal_resize",
            "terminalResize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WorkloadId,
            Stdin,
            StdinClosed,
            TerminalResize,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "workloadId" | "workload_id" => Ok(GeneratedField::WorkloadId),
                            "stdin" => Ok(GeneratedField::Stdin),
                            "stdinClosed" | "stdin_closed" => Ok(GeneratedField::StdinClosed),
                            "terminalResize" | "terminal_resize" => Ok(GeneratedField::TerminalResize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkloadConsoleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.WorkloadConsoleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkloadConsoleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut workload_id__ = None;
                let mut stdin__ = None;
                let mut stdin_closed__ = None;
                let mut terminal_resize__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::WorkloadId => {
                            if workload_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("workloadId"));
                            }
                            workload_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Stdin => {
                            if stdin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdin"));
                            }
                            stdin__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StdinClosed => {
                            if stdin_closed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdinClosed"));
                            }
                            stdin_closed__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TerminalResize => {
                            if terminal_resize__.is_some() {
                                return Err(serde::de::Error::duplicate_field("terminalResize"));
                            }
                            terminal_resize__ = map_.next_value()?;
                        }
                    }
                }
                Ok(WorkloadConsoleRequest {
                    workload_id: workload_id__.unwrap_or_default(),
                    stdin: stdin__.unwrap_or_default(),
                    stdin_closed: stdin_closed__.unwrap_or_default(),
                    terminal_resize: terminal_resize__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.WorkloadConsoleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkloadErrorStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.WorkloadErrorStatus", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkloadErrorStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkloadErrorStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.WorkloadErrorStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkloadErrorStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WorkloadErrorStatus {
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.WorkloadErrorStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkloadExitStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.WorkloadExitStatus", len)?;
        if self.code != 0 {
            struct_ser.serialize_field("code", &self.code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkloadExitStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkloadExitStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.WorkloadExitStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkloadExitStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(WorkloadExitStatus {
                    code: code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.WorkloadExitStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkloadMountInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.device_id != 0 {
            len += 1;
        }
        if !self.tag.is_empty() {
            len += 1;
        }
        if !self.host_directory.is_empty() {
            len += 1;
        }
        if !self.host_file.is_empty() {
            len += 1;
        }
        if !self.target_path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.WorkloadMountInfo", len)?;
        if self.device_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("deviceId", ToString::to_string(&self.device_id).as_str())?;
        }
        if !self.tag.is_empty() {
            struct_ser.serialize_field("tag", &self.tag)?;
        }
        if !self.host_directory.is_empty() {
            struct_ser.serialize_field("hostDirectory", &self.host_directory)?;
        }
        if !self.host_file.is_empty() {
            struct_ser.serialize_field("hostFile", &self.host_file)?;
        }
        if !self.target_path.is_empty() {
            struct_ser.serialize_field("targetPath", &self.target_path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkloadMountInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "device_id",
            "deviceId",
            "tag",
            "host_directory",
            "hostDirectory",
            "host_file",
            "hostFile",
            "target_path",
            "targetPath",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeviceId,
            Tag,
            HostDirectory,
            HostFile,
            TargetPath,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "deviceId" | "device_id" => Ok(GeneratedField::DeviceId),
                            "tag" => Ok(GeneratedField::Tag),
                            "hostDirectory" | "host_directory" => Ok(GeneratedField::HostDirectory),
                            "hostFile" | "host_file" => Ok(GeneratedField::HostFile),
                            "targetPath" | "target_path" => Ok(GeneratedField::TargetPath),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkloadMountInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.WorkloadMountInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkloadMountInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut device_id__ = None;
                let mut tag__ = None;
                let mut host_directory__ = None;
                let mut host_file__ = None;
                let mut target_path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeviceId => {
                            if device_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceId"));
                            }
                            device_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Tag => {
                            if tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tag"));
                            }
                            tag__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HostDirectory => {
                            if host_directory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostDirectory"));
                            }
                            host_directory__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HostFile => {
                            if host_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostFile"));
                            }
                            host_file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TargetPath => {
                            if target_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetPath"));
                            }
                            target_path__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WorkloadMountInfo {
                    device_id: device_id__.unwrap_or_default(),
                    tag: tag__.unwrap_or_default(),
                    host_directory: host_directory__.unwrap_or_default(),
                    host_file: host_file__.unwrap_or_default(),
                    target_path: target_path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.WorkloadMountInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkloadMountStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.devices.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.WorkloadMountStatus", len)?;
        if !self.devices.is_empty() {
            struct_ser.serialize_field("devices", &self.devices)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkloadMountStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "devices",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Devices,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "devices" => Ok(GeneratedField::Devices),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkloadMountStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.WorkloadMountStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkloadMountStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut devices__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Devices => {
                            if devices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("devices"));
                            }
                            devices__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WorkloadMountStatus {
                    devices: devices__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.WorkloadMountStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkloadScratchMount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.WorkloadScratchMount", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkloadScratchMount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "path" => Ok(GeneratedField::Path),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkloadScratchMount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.WorkloadScratchMount")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkloadScratchMount, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WorkloadScratchMount {
                    path: path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.WorkloadScratchMount", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkloadSecuritySpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.strict_user_namespace {
            len += 1;
        }
        if !self.raise_capabilities.is_empty() {
            len += 1;
        }
        if !self.raise_ambient_capabilities.is_empty() {
            len += 1;
        }
        if !self.drop_capabilities.is_empty() {
            len += 1;
        }
        if self.privileged {
            len += 1;
        }
        if self.disable_all_namespaces {
            len += 1;
        }
        if !self.disable_namespaces.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.WorkloadSecuritySpec", len)?;
        if self.strict_user_namespace {
            struct_ser.serialize_field("strictUserNamespace", &self.strict_user_namespace)?;
        }
        if !self.raise_capabilities.is_empty() {
            struct_ser.serialize_field("raiseCapabilities", &self.raise_capabilities)?;
        }
        if !self.raise_ambient_capabilities.is_empty() {
            struct_ser.serialize_field("raiseAmbientCapabilities", &self.raise_ambient_capabilities)?;
        }
        if !self.drop_capabilities.is_empty() {
            struct_ser.serialize_field("dropCapabilities", &self.drop_capabilities)?;
        }
        if self.privileged {
            struct_ser.serialize_field("privileged", &self.privileged)?;
        }
        if self.disable_all_namespaces {
            struct_ser.serialize_field("disableAllNamespaces", &self.disable_all_namespaces)?;
        }
        if !self.disable_namespaces.is_empty() {
            let v = self.disable_namespaces.iter().cloned().map(|v| {
                ProcessNamespace::try_from(v)
                    .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<std::result::Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("disableNamespaces", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkloadSecuritySpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "strict_user_namespace",
            "strictUserNamespace",
            "raise_capabilities",
            "raiseCapabilities",
            "raise_ambient_capabilities",
            "raiseAmbientCapabilities",
            "drop_capabilities",
            "dropCapabilities",
            "privileged",
            "disable_all_namespaces",
            "disableAllNamespaces",
            "disable_namespaces",
            "disableNamespaces",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StrictUserNamespace,
            RaiseCapabilities,
            RaiseAmbientCapabilities,
            DropCapabilities,
            Privileged,
            DisableAllNamespaces,
            DisableNamespaces,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "strictUserNamespace" | "strict_user_namespace" => Ok(GeneratedField::StrictUserNamespace),
                            "raiseCapabilities" | "raise_capabilities" => Ok(GeneratedField::RaiseCapabilities),
                            "raiseAmbientCapabilities" | "raise_ambient_capabilities" => Ok(GeneratedField::RaiseAmbientCapabilities),
                            "dropCapabilities" | "drop_capabilities" => Ok(GeneratedField::DropCapabilities),
                            "privileged" => Ok(GeneratedField::Privileged),
                            "disableAllNamespaces" | "disable_all_namespaces" => Ok(GeneratedField::DisableAllNamespaces),
                            "disableNamespaces" | "disable_namespaces" => Ok(GeneratedField::DisableNamespaces),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkloadSecuritySpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.WorkloadSecuritySpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkloadSecuritySpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut strict_user_namespace__ = None;
                let mut raise_capabilities__ = None;
                let mut raise_ambient_capabilities__ = None;
                let mut drop_capabilities__ = None;
                let mut privileged__ = None;
                let mut disable_all_namespaces__ = None;
                let mut disable_namespaces__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StrictUserNamespace => {
                            if strict_user_namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("strictUserNamespace"));
                            }
                            strict_user_namespace__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RaiseCapabilities => {
                            if raise_capabilities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("raiseCapabilities"));
                            }
                            raise_capabilities__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RaiseAmbientCapabilities => {
                            if raise_ambient_capabilities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("raiseAmbientCapabilities"));
                            }
                            raise_ambient_capabilities__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DropCapabilities => {
                            if drop_capabilities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dropCapabilities"));
                            }
                            drop_capabilities__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Privileged => {
                            if privileged__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privileged"));
                            }
                            privileged__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisableAllNamespaces => {
                            if disable_all_namespaces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableAllNamespaces"));
                            }
                            disable_all_namespaces__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisableNamespaces => {
                            if disable_namespaces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableNamespaces"));
                            }
                            disable_namespaces__ = Some(map_.next_value::<Vec<ProcessNamespace>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(WorkloadSecuritySpec {
                    strict_user_namespace: strict_user_namespace__.unwrap_or_default(),
                    raise_capabilities: raise_capabilities__.unwrap_or_default(),
                    raise_ambient_capabilities: raise_ambient_capabilities__.unwrap_or_default(),
                    drop_capabilities: drop_capabilities__.unwrap_or_default(),
                    privileged: privileged__.unwrap_or_default(),
                    disable_all_namespaces: disable_all_namespaces__.unwrap_or_default(),
                    disable_namespaces: disable_namespaces__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.WorkloadSecuritySpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkloadSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.zone_id.is_empty() {
            len += 1;
        }
        if self.image.is_some() {
            len += 1;
        }
        if self.process.is_some() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.mounts.is_empty() {
            len += 1;
        }
        if self.security.is_some() {
            len += 1;
        }
        if !self.scratch_mount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.WorkloadSpec", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.zone_id.is_empty() {
            struct_ser.serialize_field("zoneId", &self.zone_id)?;
        }
        if let Some(v) = self.image.as_ref() {
            struct_ser.serialize_field("image", v)?;
        }
        if let Some(v) = self.process.as_ref() {
            struct_ser.serialize_field("process", v)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.mounts.is_empty() {
            struct_ser.serialize_field("mounts", &self.mounts)?;
        }
        if let Some(v) = self.security.as_ref() {
            struct_ser.serialize_field("security", v)?;
        }
        if !self.scratch_mount.is_empty() {
            struct_ser.serialize_field("scratchMount", &self.scratch_mount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkloadSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "zone_id",
            "zoneId",
            "image",
            "process",
            "annotations",
            "mounts",
            "security",
            "scratch_mount",
            "scratchMount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ZoneId,
            Image,
            Process,
            Annotations,
            Mounts,
            Security,
            ScratchMount,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            "image" => Ok(GeneratedField::Image),
                            "process" => Ok(GeneratedField::Process),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "mounts" => Ok(GeneratedField::Mounts),
                            "security" => Ok(GeneratedField::Security),
                            "scratchMount" | "scratch_mount" => Ok(GeneratedField::ScratchMount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkloadSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.WorkloadSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkloadSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut zone_id__ = None;
                let mut image__ = None;
                let mut process__ = None;
                let mut annotations__ = None;
                let mut mounts__ = None;
                let mut security__ = None;
                let mut scratch_mount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Image => {
                            if image__.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            image__ = map_.next_value()?;
                        }
                        GeneratedField::Process => {
                            if process__.is_some() {
                                return Err(serde::de::Error::duplicate_field("process"));
                            }
                            process__ = map_.next_value()?;
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Mounts => {
                            if mounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mounts"));
                            }
                            mounts__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Security => {
                            if security__.is_some() {
                                return Err(serde::de::Error::duplicate_field("security"));
                            }
                            security__ = map_.next_value()?;
                        }
                        GeneratedField::ScratchMount => {
                            if scratch_mount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scratchMount"));
                            }
                            scratch_mount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(WorkloadSpec {
                    name: name__.unwrap_or_default(),
                    zone_id: zone_id__.unwrap_or_default(),
                    image: image__,
                    process: process__,
                    annotations: annotations__.unwrap_or_default(),
                    mounts: mounts__.unwrap_or_default(),
                    security: security__,
                    scratch_mount: scratch_mount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.WorkloadSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WorkloadState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "WORKLOAD_STATE_UNKNOWN",
            Self::Creating => "WORKLOAD_STATE_CREATING",
            Self::Created => "WORKLOAD_STATE_CREATED",
            Self::Running => "WORKLOAD_STATE_RUNNING",
            Self::Completed => "WORKLOAD_STATE_COMPLETED",
            Self::Destroying => "WORKLOAD_STATE_DESTROYING",
            Self::Destroyed => "WORKLOAD_STATE_DESTROYED",
            Self::Failed => "WORKLOAD_STATE_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for WorkloadState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "WORKLOAD_STATE_UNKNOWN",
            "WORKLOAD_STATE_CREATING",
            "WORKLOAD_STATE_CREATED",
            "WORKLOAD_STATE_RUNNING",
            "WORKLOAD_STATE_COMPLETED",
            "WORKLOAD_STATE_DESTROYING",
            "WORKLOAD_STATE_DESTROYED",
            "WORKLOAD_STATE_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkloadState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "WORKLOAD_STATE_UNKNOWN" => Ok(WorkloadState::Unknown),
                    "WORKLOAD_STATE_CREATING" => Ok(WorkloadState::Creating),
                    "WORKLOAD_STATE_CREATED" => Ok(WorkloadState::Created),
                    "WORKLOAD_STATE_RUNNING" => Ok(WorkloadState::Running),
                    "WORKLOAD_STATE_COMPLETED" => Ok(WorkloadState::Completed),
                    "WORKLOAD_STATE_DESTROYING" => Ok(WorkloadState::Destroying),
                    "WORKLOAD_STATE_DESTROYED" => Ok(WorkloadState::Destroyed),
                    "WORKLOAD_STATE_FAILED" => Ok(WorkloadState::Failed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for WorkloadStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.state != 0 {
            len += 1;
        }
        if self.exit_status.is_some() {
            len += 1;
        }
        if self.error_status.is_some() {
            len += 1;
        }
        if self.block_device_status.is_some() {
            len += 1;
        }
        if self.mount_status.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.WorkloadStatus", len)?;
        if self.state != 0 {
            let v = WorkloadState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.exit_status.as_ref() {
            struct_ser.serialize_field("exitStatus", v)?;
        }
        if let Some(v) = self.error_status.as_ref() {
            struct_ser.serialize_field("errorStatus", v)?;
        }
        if let Some(v) = self.block_device_status.as_ref() {
            struct_ser.serialize_field("blockDeviceStatus", v)?;
        }
        if let Some(v) = self.mount_status.as_ref() {
            struct_ser.serialize_field("mountStatus", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WorkloadStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "state",
            "exit_status",
            "exitStatus",
            "error_status",
            "errorStatus",
            "block_device_status",
            "blockDeviceStatus",
            "mount_status",
            "mountStatus",
            "created_at",
            "createdAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
            ExitStatus,
            ErrorStatus,
            BlockDeviceStatus,
            MountStatus,
            CreatedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "state" => Ok(GeneratedField::State),
                            "exitStatus" | "exit_status" => Ok(GeneratedField::ExitStatus),
                            "errorStatus" | "error_status" => Ok(GeneratedField::ErrorStatus),
                            "blockDeviceStatus" | "block_device_status" => Ok(GeneratedField::BlockDeviceStatus),
                            "mountStatus" | "mount_status" => Ok(GeneratedField::MountStatus),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WorkloadStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.WorkloadStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<WorkloadStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
                let mut exit_status__ = None;
                let mut error_status__ = None;
                let mut block_device_status__ = None;
                let mut mount_status__ = None;
                let mut created_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<WorkloadState>()? as i32);
                        }
                        GeneratedField::ExitStatus => {
                            if exit_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exitStatus"));
                            }
                            exit_status__ = map_.next_value()?;
                        }
                        GeneratedField::ErrorStatus => {
                            if error_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorStatus"));
                            }
                            error_status__ = map_.next_value()?;
                        }
                        GeneratedField::BlockDeviceStatus => {
                            if block_device_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockDeviceStatus"));
                            }
                            block_device_status__ = map_.next_value()?;
                        }
                        GeneratedField::MountStatus => {
                            if mount_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mountStatus"));
                            }
                            mount_status__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(WorkloadStatus {
                    state: state__.unwrap_or_default(),
                    exit_status: exit_status__,
                    error_status: error_status__,
                    block_device_status: block_device_status__,
                    mount_status: mount_status__,
                    created_at: created_at__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.WorkloadStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Zone {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.Zone", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Zone {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Spec,
            Status,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Zone;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.Zone")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Zone, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Zone {
                    id: id__.unwrap_or_default(),
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.Zone", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneChangedEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.zone.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneChangedEvent", len)?;
        if let Some(v) = self.zone.as_ref() {
            struct_ser.serialize_field("zone", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneChangedEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Zone,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zone" => Ok(GeneratedField::Zone),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneChangedEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneChangedEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneChangedEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Zone => {
                            if zone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zone"));
                            }
                            zone__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ZoneChangedEvent {
                    zone: zone__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneChangedEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneConsoleReply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneConsoleReply", len)?;
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneConsoleReply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneConsoleReply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneConsoleReply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneConsoleReply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ZoneConsoleReply {
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneConsoleReply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneConsoleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_id.is_empty() {
            len += 1;
        }
        if !self.data.is_empty() {
            len += 1;
        }
        if self.replay_history {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneConsoleRequest", len)?;
        if !self.zone_id.is_empty() {
            struct_ser.serialize_field("zoneId", &self.zone_id)?;
        }
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        if self.replay_history {
            struct_ser.serialize_field("replayHistory", &self.replay_history)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneConsoleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_id",
            "zoneId",
            "data",
            "replay_history",
            "replayHistory",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneId,
            Data,
            ReplayHistory,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            "data" => Ok(GeneratedField::Data),
                            "replayHistory" | "replay_history" => Ok(GeneratedField::ReplayHistory),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneConsoleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneConsoleRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneConsoleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_id__ = None;
                let mut data__ = None;
                let mut replay_history__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ReplayHistory => {
                            if replay_history__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replayHistory"));
                            }
                            replay_history__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZoneConsoleRequest {
                    zone_id: zone_id__.unwrap_or_default(),
                    data: data__.unwrap_or_default(),
                    replay_history: replay_history__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneConsoleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneDeviceStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.disks.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneDeviceStatus", len)?;
        if !self.disks.is_empty() {
            struct_ser.serialize_field("disks", &self.disks)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneDeviceStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "disks",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Disks,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "disks" => Ok(GeneratedField::Disks),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneDeviceStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneDeviceStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneDeviceStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut disks__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Disks => {
                            if disks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disks"));
                            }
                            disks__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZoneDeviceStatus {
                    disks: disks__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneDeviceStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneDiskStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_block_device.is_empty() {
            len += 1;
        }
        if !self.host_block_device.is_empty() {
            len += 1;
        }
        if !self.host_image_file.is_empty() {
            len += 1;
        }
        if !self.filesystem_type.is_empty() {
            len += 1;
        }
        if self.purpose != 0 {
            len += 1;
        }
        if self.delete {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneDiskStatus", len)?;
        if !self.zone_block_device.is_empty() {
            struct_ser.serialize_field("zoneBlockDevice", &self.zone_block_device)?;
        }
        if !self.host_block_device.is_empty() {
            struct_ser.serialize_field("hostBlockDevice", &self.host_block_device)?;
        }
        if !self.host_image_file.is_empty() {
            struct_ser.serialize_field("hostImageFile", &self.host_image_file)?;
        }
        if !self.filesystem_type.is_empty() {
            struct_ser.serialize_field("filesystemType", &self.filesystem_type)?;
        }
        if self.purpose != 0 {
            let v = ZoneDiskStatusDiskPurpose::try_from(self.purpose)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.purpose)))?;
            struct_ser.serialize_field("purpose", &v)?;
        }
        if self.delete {
            struct_ser.serialize_field("delete", &self.delete)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneDiskStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_block_device",
            "zoneBlockDevice",
            "host_block_device",
            "hostBlockDevice",
            "host_image_file",
            "hostImageFile",
            "filesystem_type",
            "filesystemType",
            "purpose",
            "delete",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneBlockDevice,
            HostBlockDevice,
            HostImageFile,
            FilesystemType,
            Purpose,
            Delete,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneBlockDevice" | "zone_block_device" => Ok(GeneratedField::ZoneBlockDevice),
                            "hostBlockDevice" | "host_block_device" => Ok(GeneratedField::HostBlockDevice),
                            "hostImageFile" | "host_image_file" => Ok(GeneratedField::HostImageFile),
                            "filesystemType" | "filesystem_type" => Ok(GeneratedField::FilesystemType),
                            "purpose" => Ok(GeneratedField::Purpose),
                            "delete" => Ok(GeneratedField::Delete),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneDiskStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneDiskStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneDiskStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_block_device__ = None;
                let mut host_block_device__ = None;
                let mut host_image_file__ = None;
                let mut filesystem_type__ = None;
                let mut purpose__ = None;
                let mut delete__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneBlockDevice => {
                            if zone_block_device__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneBlockDevice"));
                            }
                            zone_block_device__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HostBlockDevice => {
                            if host_block_device__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostBlockDevice"));
                            }
                            host_block_device__ = Some(map_.next_value()?);
                        }
                        GeneratedField::HostImageFile => {
                            if host_image_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostImageFile"));
                            }
                            host_image_file__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FilesystemType => {
                            if filesystem_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filesystemType"));
                            }
                            filesystem_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Purpose => {
                            if purpose__.is_some() {
                                return Err(serde::de::Error::duplicate_field("purpose"));
                            }
                            purpose__ = Some(map_.next_value::<ZoneDiskStatusDiskPurpose>()? as i32);
                        }
                        GeneratedField::Delete => {
                            if delete__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delete"));
                            }
                            delete__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZoneDiskStatus {
                    zone_block_device: zone_block_device__.unwrap_or_default(),
                    host_block_device: host_block_device__.unwrap_or_default(),
                    host_image_file: host_image_file__.unwrap_or_default(),
                    filesystem_type: filesystem_type__.unwrap_or_default(),
                    purpose: purpose__.unwrap_or_default(),
                    delete: delete__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneDiskStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneDiskStatusDiskPurpose {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "ZONE_DISK_STATUS_DISK_PURPOSE_UNKNOWN",
            Self::Addons => "ZONE_DISK_STATUS_DISK_PURPOSE_ADDONS",
            Self::Scratch => "ZONE_DISK_STATUS_DISK_PURPOSE_SCRATCH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ZoneDiskStatusDiskPurpose {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ZONE_DISK_STATUS_DISK_PURPOSE_UNKNOWN",
            "ZONE_DISK_STATUS_DISK_PURPOSE_ADDONS",
            "ZONE_DISK_STATUS_DISK_PURPOSE_SCRATCH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneDiskStatusDiskPurpose;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ZONE_DISK_STATUS_DISK_PURPOSE_UNKNOWN" => Ok(ZoneDiskStatusDiskPurpose::Unknown),
                    "ZONE_DISK_STATUS_DISK_PURPOSE_ADDONS" => Ok(ZoneDiskStatusDiskPurpose::Addons),
                    "ZONE_DISK_STATUS_DISK_PURPOSE_SCRATCH" => Ok(ZoneDiskStatusDiskPurpose::Scratch),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneErrorStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneErrorStatus", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneErrorStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneErrorStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneErrorStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneErrorStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZoneErrorStatus {
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneErrorStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneExitStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneExitStatus", len)?;
        if self.code != 0 {
            struct_ser.serialize_field("code", &self.code)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneExitStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "code" => Ok(GeneratedField::Code),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneExitStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneExitStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneExitStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ZoneExitStatus {
                    code: code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneExitStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelEventParam {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.param_type != 0 {
            len += 1;
        }
        if !self.param_data.is_empty() {
            len += 1;
        }
        if !self.param_pretty.is_empty() {
            len += 1;
        }
        if !self.param_type_pretty.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneKernelEventParam", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.param_type != 0 {
            struct_ser.serialize_field("paramType", &self.param_type)?;
        }
        if !self.param_data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("paramData", pbjson::private::base64::encode(&self.param_data).as_str())?;
        }
        if !self.param_pretty.is_empty() {
            struct_ser.serialize_field("paramPretty", &self.param_pretty)?;
        }
        if !self.param_type_pretty.is_empty() {
            struct_ser.serialize_field("paramTypePretty", &self.param_type_pretty)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelEventParam {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "param_type",
            "paramType",
            "param_data",
            "paramData",
            "param_pretty",
            "paramPretty",
            "param_type_pretty",
            "paramTypePretty",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ParamType,
            ParamData,
            ParamPretty,
            ParamTypePretty,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "paramType" | "param_type" => Ok(GeneratedField::ParamType),
                            "paramData" | "param_data" => Ok(GeneratedField::ParamData),
                            "paramPretty" | "param_pretty" => Ok(GeneratedField::ParamPretty),
                            "paramTypePretty" | "param_type_pretty" => Ok(GeneratedField::ParamTypePretty),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelEventParam;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneKernelEventParam")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneKernelEventParam, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut param_type__ = None;
                let mut param_data__ = None;
                let mut param_pretty__ = None;
                let mut param_type_pretty__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ParamType => {
                            if param_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paramType"));
                            }
                            param_type__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ParamData => {
                            if param_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paramData"));
                            }
                            param_data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ParamPretty => {
                            if param_pretty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paramPretty"));
                            }
                            param_pretty__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ParamTypePretty => {
                            if param_type_pretty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paramTypePretty"));
                            }
                            param_type_pretty__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZoneKernelEventParam {
                    name: name__.unwrap_or_default(),
                    param_type: param_type__.unwrap_or_default(),
                    param_data: param_data__.unwrap_or_default(),
                    param_pretty: param_pretty__.unwrap_or_default(),
                    param_type_pretty: param_type_pretty__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneKernelEventParam", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelEventStreamStop {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("protect.control.v1.ZoneKernelEventStreamStop", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelEventStreamStop {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelEventStreamStop;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneKernelEventStreamStop")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneKernelEventStreamStop, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ZoneKernelEventStreamStop {
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneKernelEventStreamStop", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelEventStreamUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.enabled_syscalls.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneKernelEventStreamUpdate", len)?;
        if !self.enabled_syscalls.is_empty() {
            struct_ser.serialize_field("enabledSyscalls", &self.enabled_syscalls)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelEventStreamUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enabled_syscalls",
            "enabledSyscalls",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EnabledSyscalls,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "enabledSyscalls" | "enabled_syscalls" => Ok(GeneratedField::EnabledSyscalls),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelEventStreamUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneKernelEventStreamUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneKernelEventStreamUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enabled_syscalls__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EnabledSyscalls => {
                            if enabled_syscalls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabledSyscalls"));
                            }
                            enabled_syscalls__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(ZoneKernelEventStreamUpdate {
                    enabled_syscalls: enabled_syscalls__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneKernelEventStreamUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelFdInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fd.is_some() {
            len += 1;
        }
        if self.inode != 0 {
            len += 1;
        }
        if self.fd_type != 0 {
            len += 1;
        }
        if self.info.is_some() {
            len += 1;
        }
        if self.open_flags != 0 {
            len += 1;
        }
        if self.state_flags != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneKernelFdInfo", len)?;
        if let Some(v) = self.fd.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("fd", ToString::to_string(&v).as_str())?;
        }
        if self.inode != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("inode", ToString::to_string(&self.inode).as_str())?;
        }
        if self.fd_type != 0 {
            let v = ZoneKernelFdType::try_from(self.fd_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.fd_type)))?;
            struct_ser.serialize_field("fdType", &v)?;
        }
        if let Some(v) = self.info.as_ref() {
            struct_ser.serialize_field("info", v)?;
        }
        if self.open_flags != 0 {
            struct_ser.serialize_field("openFlags", &self.open_flags)?;
        }
        if self.state_flags != 0 {
            struct_ser.serialize_field("stateFlags", &self.state_flags)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelFdInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fd",
            "inode",
            "fd_type",
            "fdType",
            "info",
            "open_flags",
            "openFlags",
            "state_flags",
            "stateFlags",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fd,
            Inode,
            FdType,
            Info,
            OpenFlags,
            StateFlags,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fd" => Ok(GeneratedField::Fd),
                            "inode" => Ok(GeneratedField::Inode),
                            "fdType" | "fd_type" => Ok(GeneratedField::FdType),
                            "info" => Ok(GeneratedField::Info),
                            "openFlags" | "open_flags" => Ok(GeneratedField::OpenFlags),
                            "stateFlags" | "state_flags" => Ok(GeneratedField::StateFlags),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelFdInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneKernelFdInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneKernelFdInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fd__ = None;
                let mut inode__ = None;
                let mut fd_type__ = None;
                let mut info__ = None;
                let mut open_flags__ = None;
                let mut state_flags__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fd => {
                            if fd__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fd"));
                            }
                            fd__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Inode => {
                            if inode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inode"));
                            }
                            inode__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FdType => {
                            if fd_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fdType"));
                            }
                            fd_type__ = Some(map_.next_value::<ZoneKernelFdType>()? as i32);
                        }
                        GeneratedField::Info => {
                            if info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            info__ = map_.next_value()?;
                        }
                        GeneratedField::OpenFlags => {
                            if open_flags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("openFlags"));
                            }
                            open_flags__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StateFlags => {
                            if state_flags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stateFlags"));
                            }
                            state_flags__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ZoneKernelFdInfo {
                    fd: fd__,
                    inode: inode__.unwrap_or_default(),
                    fd_type: fd_type__.unwrap_or_default(),
                    info: info__,
                    open_flags: open_flags__.unwrap_or_default(),
                    state_flags: state_flags__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneKernelFdInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelFdInfoData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.info_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneKernelFdInfoData", len)?;
        if let Some(v) = self.info_type.as_ref() {
            match v {
                zone_kernel_fd_info_data::InfoType::Ipv4Socket(v) => {
                    struct_ser.serialize_field("ipv4Socket", v)?;
                }
                zone_kernel_fd_info_data::InfoType::Ipv6Socket(v) => {
                    struct_ser.serialize_field("ipv6Socket", v)?;
                }
                zone_kernel_fd_info_data::InfoType::Ipv4ServerSocket(v) => {
                    struct_ser.serialize_field("ipv4ServerSocket", v)?;
                }
                zone_kernel_fd_info_data::InfoType::Ipv6ServerSocket(v) => {
                    struct_ser.serialize_field("ipv6ServerSocket", v)?;
                }
                zone_kernel_fd_info_data::InfoType::UnixSocket(v) => {
                    struct_ser.serialize_field("unixSocket", v)?;
                }
                zone_kernel_fd_info_data::InfoType::RegularFile(v) => {
                    struct_ser.serialize_field("regularFile", v)?;
                }
                zone_kernel_fd_info_data::InfoType::FileName(v) => {
                    struct_ser.serialize_field("fileName", v)?;
                }
                zone_kernel_fd_info_data::InfoType::Pidfd(v) => {
                    struct_ser.serialize_field("pidfd", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelFdInfoData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ipv4_socket",
            "ipv4Socket",
            "ipv6_socket",
            "ipv6Socket",
            "ipv4_server_socket",
            "ipv4ServerSocket",
            "ipv6_server_socket",
            "ipv6ServerSocket",
            "unix_socket",
            "unixSocket",
            "regular_file",
            "regularFile",
            "file_name",
            "fileName",
            "pidfd",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ipv4Socket,
            Ipv6Socket,
            Ipv4ServerSocket,
            Ipv6ServerSocket,
            UnixSocket,
            RegularFile,
            FileName,
            Pidfd,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ipv4Socket" | "ipv4_socket" => Ok(GeneratedField::Ipv4Socket),
                            "ipv6Socket" | "ipv6_socket" => Ok(GeneratedField::Ipv6Socket),
                            "ipv4ServerSocket" | "ipv4_server_socket" => Ok(GeneratedField::Ipv4ServerSocket),
                            "ipv6ServerSocket" | "ipv6_server_socket" => Ok(GeneratedField::Ipv6ServerSocket),
                            "unixSocket" | "unix_socket" => Ok(GeneratedField::UnixSocket),
                            "regularFile" | "regular_file" => Ok(GeneratedField::RegularFile),
                            "fileName" | "file_name" => Ok(GeneratedField::FileName),
                            "pidfd" => Ok(GeneratedField::Pidfd),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelFdInfoData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneKernelFdInfoData")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneKernelFdInfoData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut info_type__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Ipv4Socket => {
                            if info_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv4Socket"));
                            }
                            info_type__ = map_.next_value::<::std::option::Option<_>>()?.map(zone_kernel_fd_info_data::InfoType::Ipv4Socket)
;
                        }
                        GeneratedField::Ipv6Socket => {
                            if info_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv6Socket"));
                            }
                            info_type__ = map_.next_value::<::std::option::Option<_>>()?.map(zone_kernel_fd_info_data::InfoType::Ipv6Socket)
;
                        }
                        GeneratedField::Ipv4ServerSocket => {
                            if info_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv4ServerSocket"));
                            }
                            info_type__ = map_.next_value::<::std::option::Option<_>>()?.map(zone_kernel_fd_info_data::InfoType::Ipv4ServerSocket)
;
                        }
                        GeneratedField::Ipv6ServerSocket => {
                            if info_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv6ServerSocket"));
                            }
                            info_type__ = map_.next_value::<::std::option::Option<_>>()?.map(zone_kernel_fd_info_data::InfoType::Ipv6ServerSocket)
;
                        }
                        GeneratedField::UnixSocket => {
                            if info_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unixSocket"));
                            }
                            info_type__ = map_.next_value::<::std::option::Option<_>>()?.map(zone_kernel_fd_info_data::InfoType::UnixSocket)
;
                        }
                        GeneratedField::RegularFile => {
                            if info_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regularFile"));
                            }
                            info_type__ = map_.next_value::<::std::option::Option<_>>()?.map(zone_kernel_fd_info_data::InfoType::RegularFile)
;
                        }
                        GeneratedField::FileName => {
                            if info_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileName"));
                            }
                            info_type__ = map_.next_value::<::std::option::Option<_>>()?.map(zone_kernel_fd_info_data::InfoType::FileName);
                        }
                        GeneratedField::Pidfd => {
                            if info_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pidfd"));
                            }
                            info_type__ = map_.next_value::<::std::option::Option<_>>()?.map(zone_kernel_fd_info_data::InfoType::Pidfd)
;
                        }
                    }
                }
                Ok(ZoneKernelFdInfoData {
                    info_type: info_type__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneKernelFdInfoData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelFdType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "ZONE_KERNEL_FD_TYPE_UNKNOWN",
            Self::File => "ZONE_KERNEL_FD_TYPE_FILE",
            Self::Directory => "ZONE_KERNEL_FD_TYPE_DIRECTORY",
            Self::Ipv4Sock => "ZONE_KERNEL_FD_TYPE_IPV4_SOCK",
            Self::Ipv6Sock => "ZONE_KERNEL_FD_TYPE_IPV6_SOCK",
            Self::Ipv4Servsock => "ZONE_KERNEL_FD_TYPE_IPV4_SERVSOCK",
            Self::Ipv6Servsock => "ZONE_KERNEL_FD_TYPE_IPV6_SERVSOCK",
            Self::Fifo => "ZONE_KERNEL_FD_TYPE_FIFO",
            Self::UnixSock => "ZONE_KERNEL_FD_TYPE_UNIX_SOCK",
            Self::Event => "ZONE_KERNEL_FD_TYPE_EVENT",
            Self::Unsupported => "ZONE_KERNEL_FD_TYPE_UNSUPPORTED",
            Self::Signalfd => "ZONE_KERNEL_FD_TYPE_SIGNALFD",
            Self::Eventpoll => "ZONE_KERNEL_FD_TYPE_EVENTPOLL",
            Self::Inotify => "ZONE_KERNEL_FD_TYPE_INOTIFY",
            Self::Timerfd => "ZONE_KERNEL_FD_TYPE_TIMERFD",
            Self::Netlink => "ZONE_KERNEL_FD_TYPE_NETLINK",
            Self::FileV2 => "ZONE_KERNEL_FD_TYPE_FILE_V2",
            Self::Bpf => "ZONE_KERNEL_FD_TYPE_BPF",
            Self::Userfaultfd => "ZONE_KERNEL_FD_TYPE_USERFAULTFD",
            Self::Iouring => "ZONE_KERNEL_FD_TYPE_IOURING",
            Self::Memfd => "ZONE_KERNEL_FD_TYPE_MEMFD",
            Self::Pidfd => "ZONE_KERNEL_FD_TYPE_PIDFD",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelFdType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ZONE_KERNEL_FD_TYPE_UNKNOWN",
            "ZONE_KERNEL_FD_TYPE_FILE",
            "ZONE_KERNEL_FD_TYPE_DIRECTORY",
            "ZONE_KERNEL_FD_TYPE_IPV4_SOCK",
            "ZONE_KERNEL_FD_TYPE_IPV6_SOCK",
            "ZONE_KERNEL_FD_TYPE_IPV4_SERVSOCK",
            "ZONE_KERNEL_FD_TYPE_IPV6_SERVSOCK",
            "ZONE_KERNEL_FD_TYPE_FIFO",
            "ZONE_KERNEL_FD_TYPE_UNIX_SOCK",
            "ZONE_KERNEL_FD_TYPE_EVENT",
            "ZONE_KERNEL_FD_TYPE_UNSUPPORTED",
            "ZONE_KERNEL_FD_TYPE_SIGNALFD",
            "ZONE_KERNEL_FD_TYPE_EVENTPOLL",
            "ZONE_KERNEL_FD_TYPE_INOTIFY",
            "ZONE_KERNEL_FD_TYPE_TIMERFD",
            "ZONE_KERNEL_FD_TYPE_NETLINK",
            "ZONE_KERNEL_FD_TYPE_FILE_V2",
            "ZONE_KERNEL_FD_TYPE_BPF",
            "ZONE_KERNEL_FD_TYPE_USERFAULTFD",
            "ZONE_KERNEL_FD_TYPE_IOURING",
            "ZONE_KERNEL_FD_TYPE_MEMFD",
            "ZONE_KERNEL_FD_TYPE_PIDFD",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelFdType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ZONE_KERNEL_FD_TYPE_UNKNOWN" => Ok(ZoneKernelFdType::Unknown),
                    "ZONE_KERNEL_FD_TYPE_FILE" => Ok(ZoneKernelFdType::File),
                    "ZONE_KERNEL_FD_TYPE_DIRECTORY" => Ok(ZoneKernelFdType::Directory),
                    "ZONE_KERNEL_FD_TYPE_IPV4_SOCK" => Ok(ZoneKernelFdType::Ipv4Sock),
                    "ZONE_KERNEL_FD_TYPE_IPV6_SOCK" => Ok(ZoneKernelFdType::Ipv6Sock),
                    "ZONE_KERNEL_FD_TYPE_IPV4_SERVSOCK" => Ok(ZoneKernelFdType::Ipv4Servsock),
                    "ZONE_KERNEL_FD_TYPE_IPV6_SERVSOCK" => Ok(ZoneKernelFdType::Ipv6Servsock),
                    "ZONE_KERNEL_FD_TYPE_FIFO" => Ok(ZoneKernelFdType::Fifo),
                    "ZONE_KERNEL_FD_TYPE_UNIX_SOCK" => Ok(ZoneKernelFdType::UnixSock),
                    "ZONE_KERNEL_FD_TYPE_EVENT" => Ok(ZoneKernelFdType::Event),
                    "ZONE_KERNEL_FD_TYPE_UNSUPPORTED" => Ok(ZoneKernelFdType::Unsupported),
                    "ZONE_KERNEL_FD_TYPE_SIGNALFD" => Ok(ZoneKernelFdType::Signalfd),
                    "ZONE_KERNEL_FD_TYPE_EVENTPOLL" => Ok(ZoneKernelFdType::Eventpoll),
                    "ZONE_KERNEL_FD_TYPE_INOTIFY" => Ok(ZoneKernelFdType::Inotify),
                    "ZONE_KERNEL_FD_TYPE_TIMERFD" => Ok(ZoneKernelFdType::Timerfd),
                    "ZONE_KERNEL_FD_TYPE_NETLINK" => Ok(ZoneKernelFdType::Netlink),
                    "ZONE_KERNEL_FD_TYPE_FILE_V2" => Ok(ZoneKernelFdType::FileV2),
                    "ZONE_KERNEL_FD_TYPE_BPF" => Ok(ZoneKernelFdType::Bpf),
                    "ZONE_KERNEL_FD_TYPE_USERFAULTFD" => Ok(ZoneKernelFdType::Userfaultfd),
                    "ZONE_KERNEL_FD_TYPE_IOURING" => Ok(ZoneKernelFdType::Iouring),
                    "ZONE_KERNEL_FD_TYPE_MEMFD" => Ok(ZoneKernelFdType::Memfd),
                    "ZONE_KERNEL_FD_TYPE_PIDFD" => Ok(ZoneKernelFdType::Pidfd),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelIpProtocol {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "ZONE_KERNEL_IP_PROTOCOL_UNKNOWN",
            Self::Na => "ZONE_KERNEL_IP_PROTOCOL_NA",
            Self::Tcp => "ZONE_KERNEL_IP_PROTOCOL_TCP",
            Self::Udp => "ZONE_KERNEL_IP_PROTOCOL_UDP",
            Self::Icmp => "ZONE_KERNEL_IP_PROTOCOL_ICMP",
            Self::Raw => "ZONE_KERNEL_IP_PROTOCOL_RAW",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelIpProtocol {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ZONE_KERNEL_IP_PROTOCOL_UNKNOWN",
            "ZONE_KERNEL_IP_PROTOCOL_NA",
            "ZONE_KERNEL_IP_PROTOCOL_TCP",
            "ZONE_KERNEL_IP_PROTOCOL_UDP",
            "ZONE_KERNEL_IP_PROTOCOL_ICMP",
            "ZONE_KERNEL_IP_PROTOCOL_RAW",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelIpProtocol;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ZONE_KERNEL_IP_PROTOCOL_UNKNOWN" => Ok(ZoneKernelIpProtocol::Unknown),
                    "ZONE_KERNEL_IP_PROTOCOL_NA" => Ok(ZoneKernelIpProtocol::Na),
                    "ZONE_KERNEL_IP_PROTOCOL_TCP" => Ok(ZoneKernelIpProtocol::Tcp),
                    "ZONE_KERNEL_IP_PROTOCOL_UDP" => Ok(ZoneKernelIpProtocol::Udp),
                    "ZONE_KERNEL_IP_PROTOCOL_ICMP" => Ok(ZoneKernelIpProtocol::Icmp),
                    "ZONE_KERNEL_IP_PROTOCOL_RAW" => Ok(ZoneKernelIpProtocol::Raw),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelIpv4ServerSocketInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.local_ip.is_empty() {
            len += 1;
        }
        if self.local_port != 0 {
            len += 1;
        }
        if self.protocol != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneKernelIpv4ServerSocketInfo", len)?;
        if !self.local_ip.is_empty() {
            struct_ser.serialize_field("localIp", &self.local_ip)?;
        }
        if self.local_port != 0 {
            struct_ser.serialize_field("localPort", &self.local_port)?;
        }
        if self.protocol != 0 {
            let v = ZoneKernelIpProtocol::try_from(self.protocol)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.protocol)))?;
            struct_ser.serialize_field("protocol", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelIpv4ServerSocketInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "local_ip",
            "localIp",
            "local_port",
            "localPort",
            "protocol",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LocalIp,
            LocalPort,
            Protocol,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "localIp" | "local_ip" => Ok(GeneratedField::LocalIp),
                            "localPort" | "local_port" => Ok(GeneratedField::LocalPort),
                            "protocol" => Ok(GeneratedField::Protocol),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelIpv4ServerSocketInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneKernelIpv4ServerSocketInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneKernelIpv4ServerSocketInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut local_ip__ = None;
                let mut local_port__ = None;
                let mut protocol__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LocalIp => {
                            if local_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localIp"));
                            }
                            local_ip__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocalPort => {
                            if local_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localPort"));
                            }
                            local_port__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map_.next_value::<ZoneKernelIpProtocol>()? as i32);
                        }
                    }
                }
                Ok(ZoneKernelIpv4ServerSocketInfo {
                    local_ip: local_ip__.unwrap_or_default(),
                    local_port: local_port__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneKernelIpv4ServerSocketInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelIpv4SocketInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.source_ip.is_empty() {
            len += 1;
        }
        if !self.dest_ip.is_empty() {
            len += 1;
        }
        if self.source_port != 0 {
            len += 1;
        }
        if self.dest_port != 0 {
            len += 1;
        }
        if self.protocol != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneKernelIpv4SocketInfo", len)?;
        if !self.source_ip.is_empty() {
            struct_ser.serialize_field("sourceIp", &self.source_ip)?;
        }
        if !self.dest_ip.is_empty() {
            struct_ser.serialize_field("destIp", &self.dest_ip)?;
        }
        if self.source_port != 0 {
            struct_ser.serialize_field("sourcePort", &self.source_port)?;
        }
        if self.dest_port != 0 {
            struct_ser.serialize_field("destPort", &self.dest_port)?;
        }
        if self.protocol != 0 {
            let v = ZoneKernelIpProtocol::try_from(self.protocol)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.protocol)))?;
            struct_ser.serialize_field("protocol", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelIpv4SocketInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_ip",
            "sourceIp",
            "dest_ip",
            "destIp",
            "source_port",
            "sourcePort",
            "dest_port",
            "destPort",
            "protocol",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceIp,
            DestIp,
            SourcePort,
            DestPort,
            Protocol,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sourceIp" | "source_ip" => Ok(GeneratedField::SourceIp),
                            "destIp" | "dest_ip" => Ok(GeneratedField::DestIp),
                            "sourcePort" | "source_port" => Ok(GeneratedField::SourcePort),
                            "destPort" | "dest_port" => Ok(GeneratedField::DestPort),
                            "protocol" => Ok(GeneratedField::Protocol),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelIpv4SocketInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneKernelIpv4SocketInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneKernelIpv4SocketInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source_ip__ = None;
                let mut dest_ip__ = None;
                let mut source_port__ = None;
                let mut dest_port__ = None;
                let mut protocol__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SourceIp => {
                            if source_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceIp"));
                            }
                            source_ip__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DestIp => {
                            if dest_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destIp"));
                            }
                            dest_ip__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourcePort => {
                            if source_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcePort"));
                            }
                            source_port__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DestPort => {
                            if dest_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destPort"));
                            }
                            dest_port__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map_.next_value::<ZoneKernelIpProtocol>()? as i32);
                        }
                    }
                }
                Ok(ZoneKernelIpv4SocketInfo {
                    source_ip: source_ip__.unwrap_or_default(),
                    dest_ip: dest_ip__.unwrap_or_default(),
                    source_port: source_port__.unwrap_or_default(),
                    dest_port: dest_port__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneKernelIpv4SocketInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelIpv6ServerSocketInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.local_ip.is_empty() {
            len += 1;
        }
        if self.local_port != 0 {
            len += 1;
        }
        if self.protocol != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneKernelIpv6ServerSocketInfo", len)?;
        if !self.local_ip.is_empty() {
            struct_ser.serialize_field("localIp", &self.local_ip)?;
        }
        if self.local_port != 0 {
            struct_ser.serialize_field("localPort", &self.local_port)?;
        }
        if self.protocol != 0 {
            let v = ZoneKernelIpProtocol::try_from(self.protocol)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.protocol)))?;
            struct_ser.serialize_field("protocol", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelIpv6ServerSocketInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "local_ip",
            "localIp",
            "local_port",
            "localPort",
            "protocol",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LocalIp,
            LocalPort,
            Protocol,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "localIp" | "local_ip" => Ok(GeneratedField::LocalIp),
                            "localPort" | "local_port" => Ok(GeneratedField::LocalPort),
                            "protocol" => Ok(GeneratedField::Protocol),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelIpv6ServerSocketInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneKernelIpv6ServerSocketInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneKernelIpv6ServerSocketInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut local_ip__ = None;
                let mut local_port__ = None;
                let mut protocol__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LocalIp => {
                            if local_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localIp"));
                            }
                            local_ip__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LocalPort => {
                            if local_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localPort"));
                            }
                            local_port__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map_.next_value::<ZoneKernelIpProtocol>()? as i32);
                        }
                    }
                }
                Ok(ZoneKernelIpv6ServerSocketInfo {
                    local_ip: local_ip__.unwrap_or_default(),
                    local_port: local_port__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneKernelIpv6ServerSocketInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelIpv6SocketInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.source_ip.is_empty() {
            len += 1;
        }
        if !self.dest_ip.is_empty() {
            len += 1;
        }
        if self.source_port != 0 {
            len += 1;
        }
        if self.dest_port != 0 {
            len += 1;
        }
        if self.protocol != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneKernelIpv6SocketInfo", len)?;
        if !self.source_ip.is_empty() {
            struct_ser.serialize_field("sourceIp", &self.source_ip)?;
        }
        if !self.dest_ip.is_empty() {
            struct_ser.serialize_field("destIp", &self.dest_ip)?;
        }
        if self.source_port != 0 {
            struct_ser.serialize_field("sourcePort", &self.source_port)?;
        }
        if self.dest_port != 0 {
            struct_ser.serialize_field("destPort", &self.dest_port)?;
        }
        if self.protocol != 0 {
            let v = ZoneKernelIpProtocol::try_from(self.protocol)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.protocol)))?;
            struct_ser.serialize_field("protocol", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelIpv6SocketInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_ip",
            "sourceIp",
            "dest_ip",
            "destIp",
            "source_port",
            "sourcePort",
            "dest_port",
            "destPort",
            "protocol",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceIp,
            DestIp,
            SourcePort,
            DestPort,
            Protocol,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sourceIp" | "source_ip" => Ok(GeneratedField::SourceIp),
                            "destIp" | "dest_ip" => Ok(GeneratedField::DestIp),
                            "sourcePort" | "source_port" => Ok(GeneratedField::SourcePort),
                            "destPort" | "dest_port" => Ok(GeneratedField::DestPort),
                            "protocol" => Ok(GeneratedField::Protocol),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelIpv6SocketInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneKernelIpv6SocketInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneKernelIpv6SocketInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source_ip__ = None;
                let mut dest_ip__ = None;
                let mut source_port__ = None;
                let mut dest_port__ = None;
                let mut protocol__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SourceIp => {
                            if source_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceIp"));
                            }
                            source_ip__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DestIp => {
                            if dest_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destIp"));
                            }
                            dest_ip__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourcePort => {
                            if source_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcePort"));
                            }
                            source_port__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DestPort => {
                            if dest_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destPort"));
                            }
                            dest_port__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map_.next_value::<ZoneKernelIpProtocol>()? as i32);
                        }
                    }
                }
                Ok(ZoneKernelIpv6SocketInfo {
                    source_ip: source_ip__.unwrap_or_default(),
                    dest_ip: dest_ip__.unwrap_or_default(),
                    source_port: source_port__.unwrap_or_default(),
                    dest_port: dest_port__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneKernelIpv6SocketInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelPidFd {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pid != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneKernelPidFd", len)?;
        if self.pid != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pid", ToString::to_string(&self.pid).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelPidFd {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pid" => Ok(GeneratedField::Pid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelPidFd;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneKernelPidFd")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneKernelPidFd, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ZoneKernelPidFd {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneKernelPidFd", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelRegularFileInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.open_flags != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.mount_id != 0 {
            len += 1;
        }
        if self.device != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneKernelRegularFileInfo", len)?;
        if self.open_flags != 0 {
            struct_ser.serialize_field("openFlags", &self.open_flags)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.mount_id != 0 {
            struct_ser.serialize_field("mountId", &self.mount_id)?;
        }
        if self.device != 0 {
            struct_ser.serialize_field("device", &self.device)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelRegularFileInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "open_flags",
            "openFlags",
            "name",
            "mount_id",
            "mountId",
            "device",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OpenFlags,
            Name,
            MountId,
            Device,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "openFlags" | "open_flags" => Ok(GeneratedField::OpenFlags),
                            "name" => Ok(GeneratedField::Name),
                            "mountId" | "mount_id" => Ok(GeneratedField::MountId),
                            "device" => Ok(GeneratedField::Device),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelRegularFileInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneKernelRegularFileInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneKernelRegularFileInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut open_flags__ = None;
                let mut name__ = None;
                let mut mount_id__ = None;
                let mut device__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OpenFlags => {
                            if open_flags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("openFlags"));
                            }
                            open_flags__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MountId => {
                            if mount_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mountId"));
                            }
                            mount_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Device => {
                            if device__.is_some() {
                                return Err(serde::de::Error::duplicate_field("device"));
                            }
                            device__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ZoneKernelRegularFileInfo {
                    open_flags: open_flags__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    mount_id: mount_id__.unwrap_or_default(),
                    device: device__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneKernelRegularFileInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelSyscallEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_id.is_empty() {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        if self.thread_id != 0 {
            len += 1;
        }
        if self.event_length != 0 {
            len += 1;
        }
        if !self.event_name.is_empty() {
            len += 1;
        }
        if !self.event_category.is_empty() {
            len += 1;
        }
        if self.event_flags != 0 {
            len += 1;
        }
        if self.event_type != 0 {
            len += 1;
        }
        if self.cpuid != 0 {
            len += 1;
        }
        if !self.event_params.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneKernelSyscallEvent", len)?;
        if !self.zone_id.is_empty() {
            struct_ser.serialize_field("zoneId", &self.zone_id)?;
        }
        if self.timestamp != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        if self.thread_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("threadId", ToString::to_string(&self.thread_id).as_str())?;
        }
        if self.event_length != 0 {
            struct_ser.serialize_field("eventLength", &self.event_length)?;
        }
        if !self.event_name.is_empty() {
            struct_ser.serialize_field("eventName", &self.event_name)?;
        }
        if !self.event_category.is_empty() {
            struct_ser.serialize_field("eventCategory", &self.event_category)?;
        }
        if self.event_flags != 0 {
            struct_ser.serialize_field("eventFlags", &self.event_flags)?;
        }
        if self.event_type != 0 {
            struct_ser.serialize_field("eventType", &self.event_type)?;
        }
        if self.cpuid != 0 {
            struct_ser.serialize_field("cpuid", &self.cpuid)?;
        }
        if !self.event_params.is_empty() {
            struct_ser.serialize_field("eventParams", &self.event_params)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelSyscallEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_id",
            "zoneId",
            "timestamp",
            "thread_id",
            "threadId",
            "event_length",
            "eventLength",
            "event_name",
            "eventName",
            "event_category",
            "eventCategory",
            "event_flags",
            "eventFlags",
            "event_type",
            "eventType",
            "cpuid",
            "event_params",
            "eventParams",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneId,
            Timestamp,
            ThreadId,
            EventLength,
            EventName,
            EventCategory,
            EventFlags,
            EventType,
            Cpuid,
            EventParams,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "threadId" | "thread_id" => Ok(GeneratedField::ThreadId),
                            "eventLength" | "event_length" => Ok(GeneratedField::EventLength),
                            "eventName" | "event_name" => Ok(GeneratedField::EventName),
                            "eventCategory" | "event_category" => Ok(GeneratedField::EventCategory),
                            "eventFlags" | "event_flags" => Ok(GeneratedField::EventFlags),
                            "eventType" | "event_type" => Ok(GeneratedField::EventType),
                            "cpuid" => Ok(GeneratedField::Cpuid),
                            "eventParams" | "event_params" => Ok(GeneratedField::EventParams),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelSyscallEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneKernelSyscallEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneKernelSyscallEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_id__ = None;
                let mut timestamp__ = None;
                let mut thread_id__ = None;
                let mut event_length__ = None;
                let mut event_name__ = None;
                let mut event_category__ = None;
                let mut event_flags__ = None;
                let mut event_type__ = None;
                let mut cpuid__ = None;
                let mut event_params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ThreadId => {
                            if thread_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threadId"));
                            }
                            thread_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EventLength => {
                            if event_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventLength"));
                            }
                            event_length__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EventName => {
                            if event_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventName"));
                            }
                            event_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EventCategory => {
                            if event_category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventCategory"));
                            }
                            event_category__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EventFlags => {
                            if event_flags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventFlags"));
                            }
                            event_flags__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EventType => {
                            if event_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventType"));
                            }
                            event_type__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Cpuid => {
                            if cpuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpuid"));
                            }
                            cpuid__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EventParams => {
                            if event_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventParams"));
                            }
                            event_params__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZoneKernelSyscallEvent {
                    zone_id: zone_id__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                    thread_id: thread_id__.unwrap_or_default(),
                    event_length: event_length__.unwrap_or_default(),
                    event_name: event_name__.unwrap_or_default(),
                    event_category: event_category__.unwrap_or_default(),
                    event_flags: event_flags__.unwrap_or_default(),
                    event_type: event_type__.unwrap_or_default(),
                    cpuid: cpuid__.unwrap_or_default(),
                    event_params: event_params__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneKernelSyscallEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelThreadInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tid.is_some() {
            len += 1;
        }
        if self.pid.is_some() {
            len += 1;
        }
        if self.ptid.is_some() {
            len += 1;
        }
        if self.sid != 0 {
            len += 1;
        }
        if self.vpgid != 0 {
            len += 1;
        }
        if self.pgid != 0 {
            len += 1;
        }
        if !self.comm.is_empty() {
            len += 1;
        }
        if !self.exe.is_empty() {
            len += 1;
        }
        if !self.exepath.is_empty() {
            len += 1;
        }
        if self.exe_writable {
            len += 1;
        }
        if self.exe_upper_layer {
            len += 1;
        }
        if self.exe_lower_layer {
            len += 1;
        }
        if self.exe_from_memfd {
            len += 1;
        }
        if !self.args.is_empty() {
            len += 1;
        }
        if !self.env.is_empty() {
            len += 1;
        }
        if !self.cwd.is_empty() {
            len += 1;
        }
        if self.fdlimit != 0 {
            len += 1;
        }
        if self.flags != 0 {
            len += 1;
        }
        if self.uid != 0 {
            len += 1;
        }
        if self.gid != 0 {
            len += 1;
        }
        if self.cap_permitted != 0 {
            len += 1;
        }
        if self.cap_effective != 0 {
            len += 1;
        }
        if self.cap_inheritable != 0 {
            len += 1;
        }
        if self.exe_ino != 0 {
            len += 1;
        }
        if self.exe_ino_ctime != 0 {
            len += 1;
        }
        if self.exe_ino_mtime != 0 {
            len += 1;
        }
        if self.exe_ino_ctime_duration_clone_ts != 0 {
            len += 1;
        }
        if self.exe_ino_ctime_duration_pidns_start != 0 {
            len += 1;
        }
        if self.vmsize_kb != 0 {
            len += 1;
        }
        if self.vmrss_kb != 0 {
            len += 1;
        }
        if self.vmswap_kb != 0 {
            len += 1;
        }
        if self.pfmajor != 0 {
            len += 1;
        }
        if self.pfminor != 0 {
            len += 1;
        }
        if self.vtid.is_some() {
            len += 1;
        }
        if self.vpid.is_some() {
            len += 1;
        }
        if self.pidns_init_start_ts != 0 {
            len += 1;
        }
        if !self.cgroups.is_empty() {
            len += 1;
        }
        if !self.root.is_empty() {
            len += 1;
        }
        if self.filtered_out {
            len += 1;
        }
        if !self.fdlist.is_empty() {
            len += 1;
        }
        if self.clone_ts != 0 {
            len += 1;
        }
        if self.tty != 0 {
            len += 1;
        }
        if self.loginuid != 0 {
            len += 1;
        }
        if self.lastexec_ts != 0 {
            len += 1;
        }
        if self.reaper {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneKernelThreadInfo", len)?;
        if let Some(v) = self.tid.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("tid", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.pid.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pid", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.ptid.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("ptid", ToString::to_string(&v).as_str())?;
        }
        if self.sid != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("sid", ToString::to_string(&self.sid).as_str())?;
        }
        if self.vpgid != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("vpgid", ToString::to_string(&self.vpgid).as_str())?;
        }
        if self.pgid != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pgid", ToString::to_string(&self.pgid).as_str())?;
        }
        if !self.comm.is_empty() {
            struct_ser.serialize_field("comm", &self.comm)?;
        }
        if !self.exe.is_empty() {
            struct_ser.serialize_field("exe", &self.exe)?;
        }
        if !self.exepath.is_empty() {
            struct_ser.serialize_field("exepath", &self.exepath)?;
        }
        if self.exe_writable {
            struct_ser.serialize_field("exeWritable", &self.exe_writable)?;
        }
        if self.exe_upper_layer {
            struct_ser.serialize_field("exeUpperLayer", &self.exe_upper_layer)?;
        }
        if self.exe_lower_layer {
            struct_ser.serialize_field("exeLowerLayer", &self.exe_lower_layer)?;
        }
        if self.exe_from_memfd {
            struct_ser.serialize_field("exeFromMemfd", &self.exe_from_memfd)?;
        }
        if !self.args.is_empty() {
            struct_ser.serialize_field("args", &self.args)?;
        }
        if !self.env.is_empty() {
            struct_ser.serialize_field("env", &self.env)?;
        }
        if !self.cwd.is_empty() {
            struct_ser.serialize_field("cwd", &self.cwd)?;
        }
        if self.fdlimit != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("fdlimit", ToString::to_string(&self.fdlimit).as_str())?;
        }
        if self.flags != 0 {
            struct_ser.serialize_field("flags", &self.flags)?;
        }
        if self.uid != 0 {
            struct_ser.serialize_field("uid", &self.uid)?;
        }
        if self.gid != 0 {
            struct_ser.serialize_field("gid", &self.gid)?;
        }
        if self.cap_permitted != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("capPermitted", ToString::to_string(&self.cap_permitted).as_str())?;
        }
        if self.cap_effective != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("capEffective", ToString::to_string(&self.cap_effective).as_str())?;
        }
        if self.cap_inheritable != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("capInheritable", ToString::to_string(&self.cap_inheritable).as_str())?;
        }
        if self.exe_ino != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("exeIno", ToString::to_string(&self.exe_ino).as_str())?;
        }
        if self.exe_ino_ctime != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("exeInoCtime", ToString::to_string(&self.exe_ino_ctime).as_str())?;
        }
        if self.exe_ino_mtime != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("exeInoMtime", ToString::to_string(&self.exe_ino_mtime).as_str())?;
        }
        if self.exe_ino_ctime_duration_clone_ts != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("exeInoCtimeDurationCloneTs", ToString::to_string(&self.exe_ino_ctime_duration_clone_ts).as_str())?;
        }
        if self.exe_ino_ctime_duration_pidns_start != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("exeInoCtimeDurationPidnsStart", ToString::to_string(&self.exe_ino_ctime_duration_pidns_start).as_str())?;
        }
        if self.vmsize_kb != 0 {
            struct_ser.serialize_field("vmsizeKb", &self.vmsize_kb)?;
        }
        if self.vmrss_kb != 0 {
            struct_ser.serialize_field("vmrssKb", &self.vmrss_kb)?;
        }
        if self.vmswap_kb != 0 {
            struct_ser.serialize_field("vmswapKb", &self.vmswap_kb)?;
        }
        if self.pfmajor != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pfmajor", ToString::to_string(&self.pfmajor).as_str())?;
        }
        if self.pfminor != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pfminor", ToString::to_string(&self.pfminor).as_str())?;
        }
        if let Some(v) = self.vtid.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("vtid", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.vpid.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("vpid", ToString::to_string(&v).as_str())?;
        }
        if self.pidns_init_start_ts != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pidnsInitStartTs", ToString::to_string(&self.pidns_init_start_ts).as_str())?;
        }
        if !self.cgroups.is_empty() {
            struct_ser.serialize_field("cgroups", &self.cgroups)?;
        }
        if !self.root.is_empty() {
            struct_ser.serialize_field("root", &self.root)?;
        }
        if self.filtered_out {
            struct_ser.serialize_field("filteredOut", &self.filtered_out)?;
        }
        if !self.fdlist.is_empty() {
            struct_ser.serialize_field("fdlist", &self.fdlist)?;
        }
        if self.clone_ts != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("cloneTs", ToString::to_string(&self.clone_ts).as_str())?;
        }
        if self.tty != 0 {
            struct_ser.serialize_field("tty", &self.tty)?;
        }
        if self.loginuid != 0 {
            struct_ser.serialize_field("loginuid", &self.loginuid)?;
        }
        if self.lastexec_ts != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lastexecTs", ToString::to_string(&self.lastexec_ts).as_str())?;
        }
        if self.reaper {
            struct_ser.serialize_field("reaper", &self.reaper)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelThreadInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tid",
            "pid",
            "ptid",
            "sid",
            "vpgid",
            "pgid",
            "comm",
            "exe",
            "exepath",
            "exe_writable",
            "exeWritable",
            "exe_upper_layer",
            "exeUpperLayer",
            "exe_lower_layer",
            "exeLowerLayer",
            "exe_from_memfd",
            "exeFromMemfd",
            "args",
            "env",
            "cwd",
            "fdlimit",
            "flags",
            "uid",
            "gid",
            "cap_permitted",
            "capPermitted",
            "cap_effective",
            "capEffective",
            "cap_inheritable",
            "capInheritable",
            "exe_ino",
            "exeIno",
            "exe_ino_ctime",
            "exeInoCtime",
            "exe_ino_mtime",
            "exeInoMtime",
            "exe_ino_ctime_duration_clone_ts",
            "exeInoCtimeDurationCloneTs",
            "exe_ino_ctime_duration_pidns_start",
            "exeInoCtimeDurationPidnsStart",
            "vmsize_kb",
            "vmsizeKb",
            "vmrss_kb",
            "vmrssKb",
            "vmswap_kb",
            "vmswapKb",
            "pfmajor",
            "pfminor",
            "vtid",
            "vpid",
            "pidns_init_start_ts",
            "pidnsInitStartTs",
            "cgroups",
            "root",
            "filtered_out",
            "filteredOut",
            "fdlist",
            "clone_ts",
            "cloneTs",
            "tty",
            "loginuid",
            "lastexec_ts",
            "lastexecTs",
            "reaper",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Tid,
            Pid,
            Ptid,
            Sid,
            Vpgid,
            Pgid,
            Comm,
            Exe,
            Exepath,
            ExeWritable,
            ExeUpperLayer,
            ExeLowerLayer,
            ExeFromMemfd,
            Args,
            Env,
            Cwd,
            Fdlimit,
            Flags,
            Uid,
            Gid,
            CapPermitted,
            CapEffective,
            CapInheritable,
            ExeIno,
            ExeInoCtime,
            ExeInoMtime,
            ExeInoCtimeDurationCloneTs,
            ExeInoCtimeDurationPidnsStart,
            VmsizeKb,
            VmrssKb,
            VmswapKb,
            Pfmajor,
            Pfminor,
            Vtid,
            Vpid,
            PidnsInitStartTs,
            Cgroups,
            Root,
            FilteredOut,
            Fdlist,
            CloneTs,
            Tty,
            Loginuid,
            LastexecTs,
            Reaper,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tid" => Ok(GeneratedField::Tid),
                            "pid" => Ok(GeneratedField::Pid),
                            "ptid" => Ok(GeneratedField::Ptid),
                            "sid" => Ok(GeneratedField::Sid),
                            "vpgid" => Ok(GeneratedField::Vpgid),
                            "pgid" => Ok(GeneratedField::Pgid),
                            "comm" => Ok(GeneratedField::Comm),
                            "exe" => Ok(GeneratedField::Exe),
                            "exepath" => Ok(GeneratedField::Exepath),
                            "exeWritable" | "exe_writable" => Ok(GeneratedField::ExeWritable),
                            "exeUpperLayer" | "exe_upper_layer" => Ok(GeneratedField::ExeUpperLayer),
                            "exeLowerLayer" | "exe_lower_layer" => Ok(GeneratedField::ExeLowerLayer),
                            "exeFromMemfd" | "exe_from_memfd" => Ok(GeneratedField::ExeFromMemfd),
                            "args" => Ok(GeneratedField::Args),
                            "env" => Ok(GeneratedField::Env),
                            "cwd" => Ok(GeneratedField::Cwd),
                            "fdlimit" => Ok(GeneratedField::Fdlimit),
                            "flags" => Ok(GeneratedField::Flags),
                            "uid" => Ok(GeneratedField::Uid),
                            "gid" => Ok(GeneratedField::Gid),
                            "capPermitted" | "cap_permitted" => Ok(GeneratedField::CapPermitted),
                            "capEffective" | "cap_effective" => Ok(GeneratedField::CapEffective),
                            "capInheritable" | "cap_inheritable" => Ok(GeneratedField::CapInheritable),
                            "exeIno" | "exe_ino" => Ok(GeneratedField::ExeIno),
                            "exeInoCtime" | "exe_ino_ctime" => Ok(GeneratedField::ExeInoCtime),
                            "exeInoMtime" | "exe_ino_mtime" => Ok(GeneratedField::ExeInoMtime),
                            "exeInoCtimeDurationCloneTs" | "exe_ino_ctime_duration_clone_ts" => Ok(GeneratedField::ExeInoCtimeDurationCloneTs),
                            "exeInoCtimeDurationPidnsStart" | "exe_ino_ctime_duration_pidns_start" => Ok(GeneratedField::ExeInoCtimeDurationPidnsStart),
                            "vmsizeKb" | "vmsize_kb" => Ok(GeneratedField::VmsizeKb),
                            "vmrssKb" | "vmrss_kb" => Ok(GeneratedField::VmrssKb),
                            "vmswapKb" | "vmswap_kb" => Ok(GeneratedField::VmswapKb),
                            "pfmajor" => Ok(GeneratedField::Pfmajor),
                            "pfminor" => Ok(GeneratedField::Pfminor),
                            "vtid" => Ok(GeneratedField::Vtid),
                            "vpid" => Ok(GeneratedField::Vpid),
                            "pidnsInitStartTs" | "pidns_init_start_ts" => Ok(GeneratedField::PidnsInitStartTs),
                            "cgroups" => Ok(GeneratedField::Cgroups),
                            "root" => Ok(GeneratedField::Root),
                            "filteredOut" | "filtered_out" => Ok(GeneratedField::FilteredOut),
                            "fdlist" => Ok(GeneratedField::Fdlist),
                            "cloneTs" | "clone_ts" => Ok(GeneratedField::CloneTs),
                            "tty" => Ok(GeneratedField::Tty),
                            "loginuid" => Ok(GeneratedField::Loginuid),
                            "lastexecTs" | "lastexec_ts" => Ok(GeneratedField::LastexecTs),
                            "reaper" => Ok(GeneratedField::Reaper),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelThreadInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneKernelThreadInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneKernelThreadInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tid__ = None;
                let mut pid__ = None;
                let mut ptid__ = None;
                let mut sid__ = None;
                let mut vpgid__ = None;
                let mut pgid__ = None;
                let mut comm__ = None;
                let mut exe__ = None;
                let mut exepath__ = None;
                let mut exe_writable__ = None;
                let mut exe_upper_layer__ = None;
                let mut exe_lower_layer__ = None;
                let mut exe_from_memfd__ = None;
                let mut args__ = None;
                let mut env__ = None;
                let mut cwd__ = None;
                let mut fdlimit__ = None;
                let mut flags__ = None;
                let mut uid__ = None;
                let mut gid__ = None;
                let mut cap_permitted__ = None;
                let mut cap_effective__ = None;
                let mut cap_inheritable__ = None;
                let mut exe_ino__ = None;
                let mut exe_ino_ctime__ = None;
                let mut exe_ino_mtime__ = None;
                let mut exe_ino_ctime_duration_clone_ts__ = None;
                let mut exe_ino_ctime_duration_pidns_start__ = None;
                let mut vmsize_kb__ = None;
                let mut vmrss_kb__ = None;
                let mut vmswap_kb__ = None;
                let mut pfmajor__ = None;
                let mut pfminor__ = None;
                let mut vtid__ = None;
                let mut vpid__ = None;
                let mut pidns_init_start_ts__ = None;
                let mut cgroups__ = None;
                let mut root__ = None;
                let mut filtered_out__ = None;
                let mut fdlist__ = None;
                let mut clone_ts__ = None;
                let mut tty__ = None;
                let mut loginuid__ = None;
                let mut lastexec_ts__ = None;
                let mut reaper__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Tid => {
                            if tid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tid"));
                            }
                            tid__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Ptid => {
                            if ptid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ptid"));
                            }
                            ptid__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Sid => {
                            if sid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sid"));
                            }
                            sid__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Vpgid => {
                            if vpgid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vpgid"));
                            }
                            vpgid__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Pgid => {
                            if pgid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pgid"));
                            }
                            pgid__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Comm => {
                            if comm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comm"));
                            }
                            comm__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Exe => {
                            if exe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exe"));
                            }
                            exe__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Exepath => {
                            if exepath__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exepath"));
                            }
                            exepath__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExeWritable => {
                            if exe_writable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exeWritable"));
                            }
                            exe_writable__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExeUpperLayer => {
                            if exe_upper_layer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exeUpperLayer"));
                            }
                            exe_upper_layer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExeLowerLayer => {
                            if exe_lower_layer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exeLowerLayer"));
                            }
                            exe_lower_layer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExeFromMemfd => {
                            if exe_from_memfd__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exeFromMemfd"));
                            }
                            exe_from_memfd__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Args => {
                            if args__.is_some() {
                                return Err(serde::de::Error::duplicate_field("args"));
                            }
                            args__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Env => {
                            if env__.is_some() {
                                return Err(serde::de::Error::duplicate_field("env"));
                            }
                            env__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Cwd => {
                            if cwd__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cwd"));
                            }
                            cwd__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fdlimit => {
                            if fdlimit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fdlimit"));
                            }
                            fdlimit__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Flags => {
                            if flags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flags"));
                            }
                            flags__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Uid => {
                            if uid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uid"));
                            }
                            uid__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Gid => {
                            if gid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gid"));
                            }
                            gid__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CapPermitted => {
                            if cap_permitted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capPermitted"));
                            }
                            cap_permitted__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CapEffective => {
                            if cap_effective__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capEffective"));
                            }
                            cap_effective__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CapInheritable => {
                            if cap_inheritable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capInheritable"));
                            }
                            cap_inheritable__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExeIno => {
                            if exe_ino__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exeIno"));
                            }
                            exe_ino__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExeInoCtime => {
                            if exe_ino_ctime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exeInoCtime"));
                            }
                            exe_ino_ctime__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExeInoMtime => {
                            if exe_ino_mtime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exeInoMtime"));
                            }
                            exe_ino_mtime__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExeInoCtimeDurationCloneTs => {
                            if exe_ino_ctime_duration_clone_ts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exeInoCtimeDurationCloneTs"));
                            }
                            exe_ino_ctime_duration_clone_ts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExeInoCtimeDurationPidnsStart => {
                            if exe_ino_ctime_duration_pidns_start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exeInoCtimeDurationPidnsStart"));
                            }
                            exe_ino_ctime_duration_pidns_start__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::VmsizeKb => {
                            if vmsize_kb__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vmsizeKb"));
                            }
                            vmsize_kb__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::VmrssKb => {
                            if vmrss_kb__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vmrssKb"));
                            }
                            vmrss_kb__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::VmswapKb => {
                            if vmswap_kb__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vmswapKb"));
                            }
                            vmswap_kb__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Pfmajor => {
                            if pfmajor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pfmajor"));
                            }
                            pfmajor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Pfminor => {
                            if pfminor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pfminor"));
                            }
                            pfminor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Vtid => {
                            if vtid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vtid"));
                            }
                            vtid__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Vpid => {
                            if vpid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vpid"));
                            }
                            vpid__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PidnsInitStartTs => {
                            if pidns_init_start_ts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pidnsInitStartTs"));
                            }
                            pidns_init_start_ts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Cgroups => {
                            if cgroups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cgroups"));
                            }
                            cgroups__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Root => {
                            if root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("root"));
                            }
                            root__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FilteredOut => {
                            if filtered_out__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filteredOut"));
                            }
                            filtered_out__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fdlist => {
                            if fdlist__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fdlist"));
                            }
                            fdlist__ = Some(
                                map_.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<u64>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                        GeneratedField::CloneTs => {
                            if clone_ts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cloneTs"));
                            }
                            clone_ts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Tty => {
                            if tty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tty"));
                            }
                            tty__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Loginuid => {
                            if loginuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loginuid"));
                            }
                            loginuid__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LastexecTs => {
                            if lastexec_ts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastexecTs"));
                            }
                            lastexec_ts__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Reaper => {
                            if reaper__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reaper"));
                            }
                            reaper__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZoneKernelThreadInfo {
                    tid: tid__,
                    pid: pid__,
                    ptid: ptid__,
                    sid: sid__.unwrap_or_default(),
                    vpgid: vpgid__.unwrap_or_default(),
                    pgid: pgid__.unwrap_or_default(),
                    comm: comm__.unwrap_or_default(),
                    exe: exe__.unwrap_or_default(),
                    exepath: exepath__.unwrap_or_default(),
                    exe_writable: exe_writable__.unwrap_or_default(),
                    exe_upper_layer: exe_upper_layer__.unwrap_or_default(),
                    exe_lower_layer: exe_lower_layer__.unwrap_or_default(),
                    exe_from_memfd: exe_from_memfd__.unwrap_or_default(),
                    args: args__.unwrap_or_default(),
                    env: env__.unwrap_or_default(),
                    cwd: cwd__.unwrap_or_default(),
                    fdlimit: fdlimit__.unwrap_or_default(),
                    flags: flags__.unwrap_or_default(),
                    uid: uid__.unwrap_or_default(),
                    gid: gid__.unwrap_or_default(),
                    cap_permitted: cap_permitted__.unwrap_or_default(),
                    cap_effective: cap_effective__.unwrap_or_default(),
                    cap_inheritable: cap_inheritable__.unwrap_or_default(),
                    exe_ino: exe_ino__.unwrap_or_default(),
                    exe_ino_ctime: exe_ino_ctime__.unwrap_or_default(),
                    exe_ino_mtime: exe_ino_mtime__.unwrap_or_default(),
                    exe_ino_ctime_duration_clone_ts: exe_ino_ctime_duration_clone_ts__.unwrap_or_default(),
                    exe_ino_ctime_duration_pidns_start: exe_ino_ctime_duration_pidns_start__.unwrap_or_default(),
                    vmsize_kb: vmsize_kb__.unwrap_or_default(),
                    vmrss_kb: vmrss_kb__.unwrap_or_default(),
                    vmswap_kb: vmswap_kb__.unwrap_or_default(),
                    pfmajor: pfmajor__.unwrap_or_default(),
                    pfminor: pfminor__.unwrap_or_default(),
                    vtid: vtid__,
                    vpid: vpid__,
                    pidns_init_start_ts: pidns_init_start_ts__.unwrap_or_default(),
                    cgroups: cgroups__.unwrap_or_default(),
                    root: root__.unwrap_or_default(),
                    filtered_out: filtered_out__.unwrap_or_default(),
                    fdlist: fdlist__.unwrap_or_default(),
                    clone_ts: clone_ts__.unwrap_or_default(),
                    tty: tty__.unwrap_or_default(),
                    loginuid: loginuid__.unwrap_or_default(),
                    lastexec_ts: lastexec_ts__.unwrap_or_default(),
                    reaper: reaper__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneKernelThreadInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelThreadSnapshotEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_id.is_empty() {
            len += 1;
        }
        if !self.thread_info.is_empty() {
            len += 1;
        }
        if self.zone_boot_epoch != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneKernelThreadSnapshotEvent", len)?;
        if !self.zone_id.is_empty() {
            struct_ser.serialize_field("zoneId", &self.zone_id)?;
        }
        if !self.thread_info.is_empty() {
            struct_ser.serialize_field("threadInfo", &self.thread_info)?;
        }
        if self.zone_boot_epoch != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("zoneBootEpoch", ToString::to_string(&self.zone_boot_epoch).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelThreadSnapshotEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_id",
            "zoneId",
            "thread_info",
            "threadInfo",
            "zone_boot_epoch",
            "zoneBootEpoch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneId,
            ThreadInfo,
            ZoneBootEpoch,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneId" | "zone_id" => Ok(GeneratedField::ZoneId),
                            "threadInfo" | "thread_info" => Ok(GeneratedField::ThreadInfo),
                            "zoneBootEpoch" | "zone_boot_epoch" => Ok(GeneratedField::ZoneBootEpoch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelThreadSnapshotEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneKernelThreadSnapshotEvent")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneKernelThreadSnapshotEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_id__ = None;
                let mut thread_info__ = None;
                let mut zone_boot_epoch__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneId => {
                            if zone_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneId"));
                            }
                            zone_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ThreadInfo => {
                            if thread_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threadInfo"));
                            }
                            thread_info__ = Some(
                                map_.next_value::<std::collections::HashMap<::pbjson::private::NumberDeserialize<u64>, _>>()?
                                    .into_iter().map(|(k,v)| (k.0, v)).collect()
                            );
                        }
                        GeneratedField::ZoneBootEpoch => {
                            if zone_boot_epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneBootEpoch"));
                            }
                            zone_boot_epoch__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ZoneKernelThreadSnapshotEvent {
                    zone_id: zone_id__.unwrap_or_default(),
                    thread_info: thread_info__.unwrap_or_default(),
                    zone_boot_epoch: zone_boot_epoch__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneKernelThreadSnapshotEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneKernelUnixSocketInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source != 0 {
            len += 1;
        }
        if self.destination != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneKernelUnixSocketInfo", len)?;
        if self.source != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("source", ToString::to_string(&self.source).as_str())?;
        }
        if self.destination != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("destination", ToString::to_string(&self.destination).as_str())?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneKernelUnixSocketInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source",
            "destination",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Source,
            Destination,
            Name,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "source" => Ok(GeneratedField::Source),
                            "destination" => Ok(GeneratedField::Destination),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneKernelUnixSocketInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneKernelUnixSocketInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneKernelUnixSocketInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source__ = None;
                let mut destination__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Destination => {
                            if destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destination"));
                            }
                            destination__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZoneKernelUnixSocketInfo {
                    source: source__.unwrap_or_default(),
                    destination: destination__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneKernelUnixSocketInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneNetworkBackend {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Default => "ZONE_NETWORK_BACKEND_DEFAULT",
            Self::None => "ZONE_NETWORK_BACKEND_NONE",
            Self::External => "ZONE_NETWORK_BACKEND_EXTERNAL",
            Self::Passthrough => "ZONE_NETWORK_BACKEND_PASSTHROUGH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ZoneNetworkBackend {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ZONE_NETWORK_BACKEND_DEFAULT",
            "ZONE_NETWORK_BACKEND_NONE",
            "ZONE_NETWORK_BACKEND_EXTERNAL",
            "ZONE_NETWORK_BACKEND_PASSTHROUGH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneNetworkBackend;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ZONE_NETWORK_BACKEND_DEFAULT" => Ok(ZoneNetworkBackend::Default),
                    "ZONE_NETWORK_BACKEND_NONE" => Ok(ZoneNetworkBackend::None),
                    "ZONE_NETWORK_BACKEND_EXTERNAL" => Ok(ZoneNetworkBackend::External),
                    "ZONE_NETWORK_BACKEND_PASSTHROUGH" => Ok(ZoneNetworkBackend::Passthrough),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneNetworkConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.interfaces.is_empty() {
            len += 1;
        }
        if !self.routes.is_empty() {
            len += 1;
        }
        if !self.neighbors.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneNetworkConfig", len)?;
        if !self.interfaces.is_empty() {
            struct_ser.serialize_field("interfaces", &self.interfaces)?;
        }
        if !self.routes.is_empty() {
            struct_ser.serialize_field("routes", &self.routes)?;
        }
        if !self.neighbors.is_empty() {
            struct_ser.serialize_field("neighbors", &self.neighbors)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneNetworkConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "interfaces",
            "routes",
            "neighbors",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Interfaces,
            Routes,
            Neighbors,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "interfaces" => Ok(GeneratedField::Interfaces),
                            "routes" => Ok(GeneratedField::Routes),
                            "neighbors" => Ok(GeneratedField::Neighbors),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneNetworkConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneNetworkConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneNetworkConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut interfaces__ = None;
                let mut routes__ = None;
                let mut neighbors__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Interfaces => {
                            if interfaces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interfaces"));
                            }
                            interfaces__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Routes => {
                            if routes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routes"));
                            }
                            routes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Neighbors => {
                            if neighbors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("neighbors"));
                            }
                            neighbors__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZoneNetworkConfig {
                    interfaces: interfaces__.unwrap_or_default(),
                    routes: routes__.unwrap_or_default(),
                    neighbors: neighbors__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneNetworkConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneNetworkInterfaceConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.zone_interface.is_empty() {
            len += 1;
        }
        if !self.zone_mac.is_empty() {
            len += 1;
        }
        if !self.ips.is_empty() {
            len += 1;
        }
        if self.mtu != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneNetworkInterfaceConfig", len)?;
        if !self.zone_interface.is_empty() {
            struct_ser.serialize_field("zoneInterface", &self.zone_interface)?;
        }
        if !self.zone_mac.is_empty() {
            struct_ser.serialize_field("zoneMac", &self.zone_mac)?;
        }
        if !self.ips.is_empty() {
            struct_ser.serialize_field("ips", &self.ips)?;
        }
        if self.mtu != 0 {
            struct_ser.serialize_field("mtu", &self.mtu)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneNetworkInterfaceConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_interface",
            "zoneInterface",
            "zone_mac",
            "zoneMac",
            "ips",
            "mtu",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneInterface,
            ZoneMac,
            Ips,
            Mtu,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "zoneInterface" | "zone_interface" => Ok(GeneratedField::ZoneInterface),
                            "zoneMac" | "zone_mac" => Ok(GeneratedField::ZoneMac),
                            "ips" => Ok(GeneratedField::Ips),
                            "mtu" => Ok(GeneratedField::Mtu),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneNetworkInterfaceConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneNetworkInterfaceConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneNetworkInterfaceConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut zone_interface__ = None;
                let mut zone_mac__ = None;
                let mut ips__ = None;
                let mut mtu__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ZoneInterface => {
                            if zone_interface__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneInterface"));
                            }
                            zone_interface__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ZoneMac => {
                            if zone_mac__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneMac"));
                            }
                            zone_mac__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ips => {
                            if ips__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ips"));
                            }
                            ips__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Mtu => {
                            if mtu__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mtu"));
                            }
                            mtu__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ZoneNetworkInterfaceConfig {
                    zone_interface: zone_interface__.unwrap_or_default(),
                    zone_mac: zone_mac__.unwrap_or_default(),
                    ips: ips__.unwrap_or_default(),
                    mtu: mtu__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneNetworkInterfaceConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneNetworkInterfaceStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.host_interface.is_empty() {
            len += 1;
        }
        if !self.zone_interface.is_empty() {
            len += 1;
        }
        if !self.zone_mac.is_empty() {
            len += 1;
        }
        if !self.ips.is_empty() {
            len += 1;
        }
        if self.mtu != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneNetworkInterfaceStatus", len)?;
        if !self.host_interface.is_empty() {
            struct_ser.serialize_field("hostInterface", &self.host_interface)?;
        }
        if !self.zone_interface.is_empty() {
            struct_ser.serialize_field("zoneInterface", &self.zone_interface)?;
        }
        if !self.zone_mac.is_empty() {
            struct_ser.serialize_field("zoneMac", &self.zone_mac)?;
        }
        if !self.ips.is_empty() {
            struct_ser.serialize_field("ips", &self.ips)?;
        }
        if self.mtu != 0 {
            struct_ser.serialize_field("mtu", &self.mtu)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneNetworkInterfaceStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host_interface",
            "hostInterface",
            "zone_interface",
            "zoneInterface",
            "zone_mac",
            "zoneMac",
            "ips",
            "mtu",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostInterface,
            ZoneInterface,
            ZoneMac,
            Ips,
            Mtu,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hostInterface" | "host_interface" => Ok(GeneratedField::HostInterface),
                            "zoneInterface" | "zone_interface" => Ok(GeneratedField::ZoneInterface),
                            "zoneMac" | "zone_mac" => Ok(GeneratedField::ZoneMac),
                            "ips" => Ok(GeneratedField::Ips),
                            "mtu" => Ok(GeneratedField::Mtu),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneNetworkInterfaceStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneNetworkInterfaceStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneNetworkInterfaceStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host_interface__ = None;
                let mut zone_interface__ = None;
                let mut zone_mac__ = None;
                let mut ips__ = None;
                let mut mtu__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::HostInterface => {
                            if host_interface__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostInterface"));
                            }
                            host_interface__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ZoneInterface => {
                            if zone_interface__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneInterface"));
                            }
                            zone_interface__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ZoneMac => {
                            if zone_mac__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneMac"));
                            }
                            zone_mac__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Ips => {
                            if ips__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ips"));
                            }
                            ips__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Mtu => {
                            if mtu__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mtu"));
                            }
                            mtu__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ZoneNetworkInterfaceStatus {
                    host_interface: host_interface__.unwrap_or_default(),
                    zone_interface: zone_interface__.unwrap_or_default(),
                    zone_mac: zone_mac__.unwrap_or_default(),
                    ips: ips__.unwrap_or_default(),
                    mtu: mtu__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneNetworkInterfaceStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneNetworkIp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.gateway.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneNetworkIp", len)?;
        if self.version != 0 {
            let v = ZoneNetworkIpVersion::try_from(self.version)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.version)))?;
            struct_ser.serialize_field("version", &v)?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.gateway.is_empty() {
            struct_ser.serialize_field("gateway", &self.gateway)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneNetworkIp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "address",
            "gateway",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Address,
            Gateway,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "version" => Ok(GeneratedField::Version),
                            "address" => Ok(GeneratedField::Address),
                            "gateway" => Ok(GeneratedField::Gateway),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneNetworkIp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneNetworkIp")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneNetworkIp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut address__ = None;
                let mut gateway__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value::<ZoneNetworkIpVersion>()? as i32);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Gateway => {
                            if gateway__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gateway"));
                            }
                            gateway__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZoneNetworkIp {
                    version: version__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    gateway: gateway__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneNetworkIp", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneNetworkIpVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "ZONE_NETWORK_IP_VERSION_UNKNOWN",
            Self::V4 => "ZONE_NETWORK_IP_VERSION_V4",
            Self::V6 => "ZONE_NETWORK_IP_VERSION_V6",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ZoneNetworkIpVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ZONE_NETWORK_IP_VERSION_UNKNOWN",
            "ZONE_NETWORK_IP_VERSION_V4",
            "ZONE_NETWORK_IP_VERSION_V6",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneNetworkIpVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ZONE_NETWORK_IP_VERSION_UNKNOWN" => Ok(ZoneNetworkIpVersion::Unknown),
                    "ZONE_NETWORK_IP_VERSION_V4" => Ok(ZoneNetworkIpVersion::V4),
                    "ZONE_NETWORK_IP_VERSION_V6" => Ok(ZoneNetworkIpVersion::V6),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneNetworkNudMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Permanent => "ZONE_NETWORK_NUD_MODE_PERMANENT",
            Self::Noarp => "ZONE_NETWORK_NUD_MODE_NOARP",
            Self::Reachable => "ZONE_NETWORK_NUD_MODE_REACHABLE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ZoneNetworkNudMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ZONE_NETWORK_NUD_MODE_PERMANENT",
            "ZONE_NETWORK_NUD_MODE_NOARP",
            "ZONE_NETWORK_NUD_MODE_REACHABLE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneNetworkNudMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ZONE_NETWORK_NUD_MODE_PERMANENT" => Ok(ZoneNetworkNudMode::Permanent),
                    "ZONE_NETWORK_NUD_MODE_NOARP" => Ok(ZoneNetworkNudMode::Noarp),
                    "ZONE_NETWORK_NUD_MODE_REACHABLE" => Ok(ZoneNetworkNudMode::Reachable),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneNetworkNeighborEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if !self.ip_address.is_empty() {
            len += 1;
        }
        if !self.link_layer_address.is_empty() {
            len += 1;
        }
        if !self.attached_interface.is_empty() {
            len += 1;
        }
        if self.unreachability_detection != 0 {
            len += 1;
        }
        if self.kind != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneNetworkNeighborEntry", len)?;
        if self.version != 0 {
            let v = ZoneNetworkIpVersion::try_from(self.version)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.version)))?;
            struct_ser.serialize_field("version", &v)?;
        }
        if !self.ip_address.is_empty() {
            struct_ser.serialize_field("ipAddress", &self.ip_address)?;
        }
        if !self.link_layer_address.is_empty() {
            struct_ser.serialize_field("linkLayerAddress", &self.link_layer_address)?;
        }
        if !self.attached_interface.is_empty() {
            struct_ser.serialize_field("attachedInterface", &self.attached_interface)?;
        }
        if self.unreachability_detection != 0 {
            let v = ZoneNetworkNudMode::try_from(self.unreachability_detection)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.unreachability_detection)))?;
            struct_ser.serialize_field("unreachabilityDetection", &v)?;
        }
        if self.kind != 0 {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneNetworkNeighborEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "ip_address",
            "ipAddress",
            "link_layer_address",
            "linkLayerAddress",
            "attached_interface",
            "attachedInterface",
            "unreachability_detection",
            "unreachabilityDetection",
            "kind",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            IpAddress,
            LinkLayerAddress,
            AttachedInterface,
            UnreachabilityDetection,
            Kind,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "version" => Ok(GeneratedField::Version),
                            "ipAddress" | "ip_address" => Ok(GeneratedField::IpAddress),
                            "linkLayerAddress" | "link_layer_address" => Ok(GeneratedField::LinkLayerAddress),
                            "attachedInterface" | "attached_interface" => Ok(GeneratedField::AttachedInterface),
                            "unreachabilityDetection" | "unreachability_detection" => Ok(GeneratedField::UnreachabilityDetection),
                            "kind" => Ok(GeneratedField::Kind),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneNetworkNeighborEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneNetworkNeighborEntry")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneNetworkNeighborEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut ip_address__ = None;
                let mut link_layer_address__ = None;
                let mut attached_interface__ = None;
                let mut unreachability_detection__ = None;
                let mut kind__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value::<ZoneNetworkIpVersion>()? as i32);
                        }
                        GeneratedField::IpAddress => {
                            if ip_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipAddress"));
                            }
                            ip_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LinkLayerAddress => {
                            if link_layer_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("linkLayerAddress"));
                            }
                            link_layer_address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AttachedInterface => {
                            if attached_interface__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachedInterface"));
                            }
                            attached_interface__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UnreachabilityDetection => {
                            if unreachability_detection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unreachabilityDetection"));
                            }
                            unreachability_detection__ = Some(map_.next_value::<ZoneNetworkNudMode>()? as i32);
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ZoneNetworkNeighborEntry {
                    version: version__.unwrap_or_default(),
                    ip_address: ip_address__.unwrap_or_default(),
                    link_layer_address: link_layer_address__.unwrap_or_default(),
                    attached_interface: attached_interface__.unwrap_or_default(),
                    unreachability_detection: unreachability_detection__.unwrap_or_default(),
                    kind: kind__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneNetworkNeighborEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneNetworkOptionsDnsSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.nameservers.is_empty() {
            len += 1;
        }
        if !self.searches.is_empty() {
            len += 1;
        }
        if !self.options.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneNetworkOptionsDnsSpec", len)?;
        if !self.nameservers.is_empty() {
            struct_ser.serialize_field("nameservers", &self.nameservers)?;
        }
        if !self.searches.is_empty() {
            struct_ser.serialize_field("searches", &self.searches)?;
        }
        if !self.options.is_empty() {
            struct_ser.serialize_field("options", &self.options)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneNetworkOptionsDnsSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nameservers",
            "searches",
            "options",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nameservers,
            Searches,
            Options,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nameservers" => Ok(GeneratedField::Nameservers),
                            "searches" => Ok(GeneratedField::Searches),
                            "options" => Ok(GeneratedField::Options),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneNetworkOptionsDnsSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneNetworkOptionsDnsSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneNetworkOptionsDnsSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nameservers__ = None;
                let mut searches__ = None;
                let mut options__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nameservers => {
                            if nameservers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nameservers"));
                            }
                            nameservers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Searches => {
                            if searches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("searches"));
                            }
                            searches__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZoneNetworkOptionsDnsSpec {
                    nameservers: nameservers__.unwrap_or_default(),
                    searches: searches__.unwrap_or_default(),
                    options: options__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneNetworkOptionsDnsSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneNetworkOptionsSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.assume_network_reservation.is_empty() {
            len += 1;
        }
        if self.retain_network_reservation {
            len += 1;
        }
        if self.dns_config.is_some() {
            len += 1;
        }
        if self.backend != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneNetworkOptionsSpec", len)?;
        if !self.assume_network_reservation.is_empty() {
            struct_ser.serialize_field("assumeNetworkReservation", &self.assume_network_reservation)?;
        }
        if self.retain_network_reservation {
            struct_ser.serialize_field("retainNetworkReservation", &self.retain_network_reservation)?;
        }
        if let Some(v) = self.dns_config.as_ref() {
            struct_ser.serialize_field("dnsConfig", v)?;
        }
        if self.backend != 0 {
            let v = ZoneNetworkBackend::try_from(self.backend)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.backend)))?;
            struct_ser.serialize_field("backend", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneNetworkOptionsSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "assume_network_reservation",
            "assumeNetworkReservation",
            "retain_network_reservation",
            "retainNetworkReservation",
            "dns_config",
            "dnsConfig",
            "backend",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AssumeNetworkReservation,
            RetainNetworkReservation,
            DnsConfig,
            Backend,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "assumeNetworkReservation" | "assume_network_reservation" => Ok(GeneratedField::AssumeNetworkReservation),
                            "retainNetworkReservation" | "retain_network_reservation" => Ok(GeneratedField::RetainNetworkReservation),
                            "dnsConfig" | "dns_config" => Ok(GeneratedField::DnsConfig),
                            "backend" => Ok(GeneratedField::Backend),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneNetworkOptionsSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneNetworkOptionsSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneNetworkOptionsSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut assume_network_reservation__ = None;
                let mut retain_network_reservation__ = None;
                let mut dns_config__ = None;
                let mut backend__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AssumeNetworkReservation => {
                            if assume_network_reservation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assumeNetworkReservation"));
                            }
                            assume_network_reservation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetainNetworkReservation => {
                            if retain_network_reservation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retainNetworkReservation"));
                            }
                            retain_network_reservation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DnsConfig => {
                            if dns_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsConfig"));
                            }
                            dns_config__ = map_.next_value()?;
                        }
                        GeneratedField::Backend => {
                            if backend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backend"));
                            }
                            backend__ = Some(map_.next_value::<ZoneNetworkBackend>()? as i32);
                        }
                    }
                }
                Ok(ZoneNetworkOptionsSpec {
                    assume_network_reservation: assume_network_reservation__.unwrap_or_default(),
                    retain_network_reservation: retain_network_reservation__.unwrap_or_default(),
                    dns_config: dns_config__,
                    backend: backend__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneNetworkOptionsSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneNetworkRoute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if !self.source.is_empty() {
            len += 1;
        }
        if !self.destination.is_empty() {
            len += 1;
        }
        if !self.gateway.is_empty() {
            len += 1;
        }
        if !self.output_interface.is_empty() {
            len += 1;
        }
        if self.scope != 0 {
            len += 1;
        }
        if self.table != 0 {
            len += 1;
        }
        if self.protocol != 0 {
            len += 1;
        }
        if self.kind != 0 {
            len += 1;
        }
        if !self.pref_source.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneNetworkRoute", len)?;
        if self.version != 0 {
            let v = ZoneNetworkIpVersion::try_from(self.version)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.version)))?;
            struct_ser.serialize_field("version", &v)?;
        }
        if !self.source.is_empty() {
            struct_ser.serialize_field("source", &self.source)?;
        }
        if !self.destination.is_empty() {
            struct_ser.serialize_field("destination", &self.destination)?;
        }
        if !self.gateway.is_empty() {
            struct_ser.serialize_field("gateway", &self.gateway)?;
        }
        if !self.output_interface.is_empty() {
            struct_ser.serialize_field("outputInterface", &self.output_interface)?;
        }
        if self.scope != 0 {
            struct_ser.serialize_field("scope", &self.scope)?;
        }
        if self.table != 0 {
            struct_ser.serialize_field("table", &self.table)?;
        }
        if self.protocol != 0 {
            struct_ser.serialize_field("protocol", &self.protocol)?;
        }
        if self.kind != 0 {
            struct_ser.serialize_field("kind", &self.kind)?;
        }
        if !self.pref_source.is_empty() {
            struct_ser.serialize_field("prefSource", &self.pref_source)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneNetworkRoute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "source",
            "destination",
            "gateway",
            "output_interface",
            "outputInterface",
            "scope",
            "table",
            "protocol",
            "kind",
            "pref_source",
            "prefSource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Source,
            Destination,
            Gateway,
            OutputInterface,
            Scope,
            Table,
            Protocol,
            Kind,
            PrefSource,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "version" => Ok(GeneratedField::Version),
                            "source" => Ok(GeneratedField::Source),
                            "destination" => Ok(GeneratedField::Destination),
                            "gateway" => Ok(GeneratedField::Gateway),
                            "outputInterface" | "output_interface" => Ok(GeneratedField::OutputInterface),
                            "scope" => Ok(GeneratedField::Scope),
                            "table" => Ok(GeneratedField::Table),
                            "protocol" => Ok(GeneratedField::Protocol),
                            "kind" => Ok(GeneratedField::Kind),
                            "prefSource" | "pref_source" => Ok(GeneratedField::PrefSource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneNetworkRoute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneNetworkRoute")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneNetworkRoute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut source__ = None;
                let mut destination__ = None;
                let mut gateway__ = None;
                let mut output_interface__ = None;
                let mut scope__ = None;
                let mut table__ = None;
                let mut protocol__ = None;
                let mut kind__ = None;
                let mut pref_source__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value::<ZoneNetworkIpVersion>()? as i32);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Destination => {
                            if destination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destination"));
                            }
                            destination__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Gateway => {
                            if gateway__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gateway"));
                            }
                            gateway__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OutputInterface => {
                            if output_interface__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputInterface"));
                            }
                            output_interface__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Scope => {
                            if scope__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            scope__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Table => {
                            if table__.is_some() {
                                return Err(serde::de::Error::duplicate_field("table"));
                            }
                            table__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PrefSource => {
                            if pref_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefSource"));
                            }
                            pref_source__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZoneNetworkRoute {
                    version: version__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                    destination: destination__.unwrap_or_default(),
                    gateway: gateway__.unwrap_or_default(),
                    output_interface: output_interface__.unwrap_or_default(),
                    scope: scope__.unwrap_or_default(),
                    table: table__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                    kind: kind__.unwrap_or_default(),
                    pref_source: pref_source__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneNetworkRoute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneNetworkStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.interfaces.is_empty() {
            len += 1;
        }
        if !self.routes.is_empty() {
            len += 1;
        }
        if !self.neighbors.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneNetworkStatus", len)?;
        if !self.interfaces.is_empty() {
            struct_ser.serialize_field("interfaces", &self.interfaces)?;
        }
        if !self.routes.is_empty() {
            struct_ser.serialize_field("routes", &self.routes)?;
        }
        if !self.neighbors.is_empty() {
            struct_ser.serialize_field("neighbors", &self.neighbors)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneNetworkStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "interfaces",
            "routes",
            "neighbors",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Interfaces,
            Routes,
            Neighbors,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "interfaces" => Ok(GeneratedField::Interfaces),
                            "routes" => Ok(GeneratedField::Routes),
                            "neighbors" => Ok(GeneratedField::Neighbors),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneNetworkStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneNetworkStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneNetworkStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut interfaces__ = None;
                let mut routes__ = None;
                let mut neighbors__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Interfaces => {
                            if interfaces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interfaces"));
                            }
                            interfaces__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Routes => {
                            if routes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routes"));
                            }
                            routes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Neighbors => {
                            if neighbors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("neighbors"));
                            }
                            neighbors__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZoneNetworkStatus {
                    interfaces: interfaces__.unwrap_or_default(),
                    routes: routes__.unwrap_or_default(),
                    neighbors: neighbors__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneNetworkStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneResourceAdjustmentPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "ZONE_RESOURCE_ADJUSTMENT_POLICY_UNKNOWN",
            Self::Static => "ZONE_RESOURCE_ADJUSTMENT_POLICY_STATIC",
            Self::Dynamic => "ZONE_RESOURCE_ADJUSTMENT_POLICY_DYNAMIC",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ZoneResourceAdjustmentPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ZONE_RESOURCE_ADJUSTMENT_POLICY_UNKNOWN",
            "ZONE_RESOURCE_ADJUSTMENT_POLICY_STATIC",
            "ZONE_RESOURCE_ADJUSTMENT_POLICY_DYNAMIC",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneResourceAdjustmentPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ZONE_RESOURCE_ADJUSTMENT_POLICY_UNKNOWN" => Ok(ZoneResourceAdjustmentPolicy::Unknown),
                    "ZONE_RESOURCE_ADJUSTMENT_POLICY_STATIC" => Ok(ZoneResourceAdjustmentPolicy::Static),
                    "ZONE_RESOURCE_ADJUSTMENT_POLICY_DYNAMIC" => Ok(ZoneResourceAdjustmentPolicy::Dynamic),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneResourceStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.active_resources.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneResourceStatus", len)?;
        if let Some(v) = self.active_resources.as_ref() {
            struct_ser.serialize_field("activeResources", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneResourceStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "active_resources",
            "activeResources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ActiveResources,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "activeResources" | "active_resources" => Ok(GeneratedField::ActiveResources),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneResourceStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneResourceStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneResourceStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut active_resources__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ActiveResources => {
                            if active_resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeResources"));
                            }
                            active_resources__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ZoneResourceStatus {
                    active_resources: active_resources__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneResourceStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneResourcesSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_memory != 0 {
            len += 1;
        }
        if self.min_memory != 0 {
            len += 1;
        }
        if self.target_memory != 0 {
            len += 1;
        }
        if self.max_cpus != 0 {
            len += 1;
        }
        if self.min_cpus != 0 {
            len += 1;
        }
        if self.target_cpus != 0 {
            len += 1;
        }
        if self.adjustment_policy != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneResourcesSpec", len)?;
        if self.max_memory != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxMemory", ToString::to_string(&self.max_memory).as_str())?;
        }
        if self.min_memory != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("minMemory", ToString::to_string(&self.min_memory).as_str())?;
        }
        if self.target_memory != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("targetMemory", ToString::to_string(&self.target_memory).as_str())?;
        }
        if self.max_cpus != 0 {
            struct_ser.serialize_field("maxCpus", &self.max_cpus)?;
        }
        if self.min_cpus != 0 {
            struct_ser.serialize_field("minCpus", &self.min_cpus)?;
        }
        if self.target_cpus != 0 {
            struct_ser.serialize_field("targetCpus", &self.target_cpus)?;
        }
        if self.adjustment_policy != 0 {
            let v = ZoneResourceAdjustmentPolicy::try_from(self.adjustment_policy)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.adjustment_policy)))?;
            struct_ser.serialize_field("adjustmentPolicy", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneResourcesSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_memory",
            "maxMemory",
            "min_memory",
            "minMemory",
            "target_memory",
            "targetMemory",
            "max_cpus",
            "maxCpus",
            "min_cpus",
            "minCpus",
            "target_cpus",
            "targetCpus",
            "adjustment_policy",
            "adjustmentPolicy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxMemory,
            MinMemory,
            TargetMemory,
            MaxCpus,
            MinCpus,
            TargetCpus,
            AdjustmentPolicy,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maxMemory" | "max_memory" => Ok(GeneratedField::MaxMemory),
                            "minMemory" | "min_memory" => Ok(GeneratedField::MinMemory),
                            "targetMemory" | "target_memory" => Ok(GeneratedField::TargetMemory),
                            "maxCpus" | "max_cpus" => Ok(GeneratedField::MaxCpus),
                            "minCpus" | "min_cpus" => Ok(GeneratedField::MinCpus),
                            "targetCpus" | "target_cpus" => Ok(GeneratedField::TargetCpus),
                            "adjustmentPolicy" | "adjustment_policy" => Ok(GeneratedField::AdjustmentPolicy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneResourcesSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneResourcesSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneResourcesSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_memory__ = None;
                let mut min_memory__ = None;
                let mut target_memory__ = None;
                let mut max_cpus__ = None;
                let mut min_cpus__ = None;
                let mut target_cpus__ = None;
                let mut adjustment_policy__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxMemory => {
                            if max_memory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxMemory"));
                            }
                            max_memory__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinMemory => {
                            if min_memory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minMemory"));
                            }
                            min_memory__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TargetMemory => {
                            if target_memory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetMemory"));
                            }
                            target_memory__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxCpus => {
                            if max_cpus__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxCpus"));
                            }
                            max_cpus__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinCpus => {
                            if min_cpus__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minCpus"));
                            }
                            min_cpus__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TargetCpus => {
                            if target_cpus__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetCpus"));
                            }
                            target_cpus__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AdjustmentPolicy => {
                            if adjustment_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjustmentPolicy"));
                            }
                            adjustment_policy__ = Some(map_.next_value::<ZoneResourceAdjustmentPolicy>()? as i32);
                        }
                    }
                }
                Ok(ZoneResourcesSpec {
                    max_memory: max_memory__.unwrap_or_default(),
                    min_memory: min_memory__.unwrap_or_default(),
                    target_memory: target_memory__.unwrap_or_default(),
                    max_cpus: max_cpus__.unwrap_or_default(),
                    min_cpus: min_cpus__.unwrap_or_default(),
                    target_cpus: target_cpus__.unwrap_or_default(),
                    adjustment_policy: adjustment_policy__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneResourcesSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneScratchDiskSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.backend.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneScratchDiskSpec", len)?;
        if let Some(v) = self.backend.as_ref() {
            match v {
                zone_scratch_disk_spec::Backend::Image(v) => {
                    struct_ser.serialize_field("image", v)?;
                }
                zone_scratch_disk_spec::Backend::Block(v) => {
                    struct_ser.serialize_field("block", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneScratchDiskSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "image",
            "block",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Image,
            Block,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "image" => Ok(GeneratedField::Image),
                            "block" => Ok(GeneratedField::Block),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneScratchDiskSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneScratchDiskSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneScratchDiskSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut backend__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Image => {
                            if backend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            backend__ = map_.next_value::<::std::option::Option<_>>()?.map(zone_scratch_disk_spec::Backend::Image)
;
                        }
                        GeneratedField::Block => {
                            if backend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("block"));
                            }
                            backend__ = map_.next_value::<::std::option::Option<_>>()?.map(zone_scratch_disk_spec::Backend::Block)
;
                        }
                    }
                }
                Ok(ZoneScratchDiskSpec {
                    backend: backend__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneScratchDiskSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneScratchDiskSpecImage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneScratchDiskSpecImage", len)?;
        if self.size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("size", ToString::to_string(&self.size).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneScratchDiskSpecImage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "size",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Size,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "size" => Ok(GeneratedField::Size),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneScratchDiskSpecImage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneScratchDiskSpecImage")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneScratchDiskSpecImage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ZoneScratchDiskSpecImage {
                    size: size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneScratchDiskSpecImage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneScratchDiskSpecStaticBlock {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.device.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneScratchDiskSpecStaticBlock", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.device.is_empty() {
            struct_ser.serialize_field("device", &self.device)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneScratchDiskSpecStaticBlock {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "device",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            Device,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "path" => Ok(GeneratedField::Path),
                            "device" => Ok(GeneratedField::Device),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneScratchDiskSpecStaticBlock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneScratchDiskSpecStaticBlock")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneScratchDiskSpecStaticBlock, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut device__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Device => {
                            if device__.is_some() {
                                return Err(serde::de::Error::duplicate_field("device"));
                            }
                            device__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZoneScratchDiskSpecStaticBlock {
                    path: path__.unwrap_or_default(),
                    device: device__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneScratchDiskSpecStaticBlock", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.kernel.is_some() {
            len += 1;
        }
        if self.initrd.is_some() {
            len += 1;
        }
        if self.kernel_options.is_some() {
            len += 1;
        }
        if self.initial_resources.is_some() {
            len += 1;
        }
        if !self.annotations.is_empty() {
            len += 1;
        }
        if !self.devices.is_empty() {
            len += 1;
        }
        if self.network_options.is_some() {
            len += 1;
        }
        if self.virtualization_options.is_some() {
            len += 1;
        }
        if self.scratch_disk.is_some() {
            len += 1;
        }
        if !self.addons.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneSpec", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.kernel.as_ref() {
            struct_ser.serialize_field("kernel", v)?;
        }
        if let Some(v) = self.initrd.as_ref() {
            struct_ser.serialize_field("initrd", v)?;
        }
        if let Some(v) = self.kernel_options.as_ref() {
            struct_ser.serialize_field("kernelOptions", v)?;
        }
        if let Some(v) = self.initial_resources.as_ref() {
            struct_ser.serialize_field("initialResources", v)?;
        }
        if !self.annotations.is_empty() {
            struct_ser.serialize_field("annotations", &self.annotations)?;
        }
        if !self.devices.is_empty() {
            struct_ser.serialize_field("devices", &self.devices)?;
        }
        if let Some(v) = self.network_options.as_ref() {
            struct_ser.serialize_field("networkOptions", v)?;
        }
        if let Some(v) = self.virtualization_options.as_ref() {
            struct_ser.serialize_field("virtualizationOptions", v)?;
        }
        if let Some(v) = self.scratch_disk.as_ref() {
            struct_ser.serialize_field("scratchDisk", v)?;
        }
        if !self.addons.is_empty() {
            struct_ser.serialize_field("addons", &self.addons)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "kernel",
            "initrd",
            "kernel_options",
            "kernelOptions",
            "initial_resources",
            "initialResources",
            "annotations",
            "devices",
            "network_options",
            "networkOptions",
            "virtualization_options",
            "virtualizationOptions",
            "scratch_disk",
            "scratchDisk",
            "addons",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Kernel,
            Initrd,
            KernelOptions,
            InitialResources,
            Annotations,
            Devices,
            NetworkOptions,
            VirtualizationOptions,
            ScratchDisk,
            Addons,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "kernel" => Ok(GeneratedField::Kernel),
                            "initrd" => Ok(GeneratedField::Initrd),
                            "kernelOptions" | "kernel_options" => Ok(GeneratedField::KernelOptions),
                            "initialResources" | "initial_resources" => Ok(GeneratedField::InitialResources),
                            "annotations" => Ok(GeneratedField::Annotations),
                            "devices" => Ok(GeneratedField::Devices),
                            "networkOptions" | "network_options" => Ok(GeneratedField::NetworkOptions),
                            "virtualizationOptions" | "virtualization_options" => Ok(GeneratedField::VirtualizationOptions),
                            "scratchDisk" | "scratch_disk" => Ok(GeneratedField::ScratchDisk),
                            "addons" => Ok(GeneratedField::Addons),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut kernel__ = None;
                let mut initrd__ = None;
                let mut kernel_options__ = None;
                let mut initial_resources__ = None;
                let mut annotations__ = None;
                let mut devices__ = None;
                let mut network_options__ = None;
                let mut virtualization_options__ = None;
                let mut scratch_disk__ = None;
                let mut addons__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Kernel => {
                            if kernel__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kernel"));
                            }
                            kernel__ = map_.next_value()?;
                        }
                        GeneratedField::Initrd => {
                            if initrd__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initrd"));
                            }
                            initrd__ = map_.next_value()?;
                        }
                        GeneratedField::KernelOptions => {
                            if kernel_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kernelOptions"));
                            }
                            kernel_options__ = map_.next_value()?;
                        }
                        GeneratedField::InitialResources => {
                            if initial_resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialResources"));
                            }
                            initial_resources__ = map_.next_value()?;
                        }
                        GeneratedField::Annotations => {
                            if annotations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("annotations"));
                            }
                            annotations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Devices => {
                            if devices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("devices"));
                            }
                            devices__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NetworkOptions => {
                            if network_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("networkOptions"));
                            }
                            network_options__ = map_.next_value()?;
                        }
                        GeneratedField::VirtualizationOptions => {
                            if virtualization_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualizationOptions"));
                            }
                            virtualization_options__ = map_.next_value()?;
                        }
                        GeneratedField::ScratchDisk => {
                            if scratch_disk__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scratchDisk"));
                            }
                            scratch_disk__ = map_.next_value()?;
                        }
                        GeneratedField::Addons => {
                            if addons__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addons"));
                            }
                            addons__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ZoneSpec {
                    name: name__.unwrap_or_default(),
                    kernel: kernel__,
                    initrd: initrd__,
                    kernel_options: kernel_options__,
                    initial_resources: initial_resources__,
                    annotations: annotations__.unwrap_or_default(),
                    devices: devices__.unwrap_or_default(),
                    network_options: network_options__,
                    virtualization_options: virtualization_options__,
                    scratch_disk: scratch_disk__,
                    addons: addons__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "ZONE_STATE_UNKNOWN",
            Self::Creating => "ZONE_STATE_CREATING",
            Self::Created => "ZONE_STATE_CREATED",
            Self::Ready => "ZONE_STATE_READY",
            Self::Exited => "ZONE_STATE_EXITED",
            Self::Destroying => "ZONE_STATE_DESTROYING",
            Self::Destroyed => "ZONE_STATE_DESTROYED",
            Self::Failed => "ZONE_STATE_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ZoneState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ZONE_STATE_UNKNOWN",
            "ZONE_STATE_CREATING",
            "ZONE_STATE_CREATED",
            "ZONE_STATE_READY",
            "ZONE_STATE_EXITED",
            "ZONE_STATE_DESTROYING",
            "ZONE_STATE_DESTROYED",
            "ZONE_STATE_FAILED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ZONE_STATE_UNKNOWN" => Ok(ZoneState::Unknown),
                    "ZONE_STATE_CREATING" => Ok(ZoneState::Creating),
                    "ZONE_STATE_CREATED" => Ok(ZoneState::Created),
                    "ZONE_STATE_READY" => Ok(ZoneState::Ready),
                    "ZONE_STATE_EXITED" => Ok(ZoneState::Exited),
                    "ZONE_STATE_DESTROYING" => Ok(ZoneState::Destroying),
                    "ZONE_STATE_DESTROYED" => Ok(ZoneState::Destroyed),
                    "ZONE_STATE_FAILED" => Ok(ZoneState::Failed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.state != 0 {
            len += 1;
        }
        if self.network_status.is_some() {
            len += 1;
        }
        if self.exit_status.is_some() {
            len += 1;
        }
        if self.error_status.is_some() {
            len += 1;
        }
        if !self.host.is_empty() {
            len += 1;
        }
        if self.domid != 0 {
            len += 1;
        }
        if self.resource_status.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.device_status.is_some() {
            len += 1;
        }
        if self.ready_at.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneStatus", len)?;
        if self.state != 0 {
            let v = ZoneState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.network_status.as_ref() {
            struct_ser.serialize_field("networkStatus", v)?;
        }
        if let Some(v) = self.exit_status.as_ref() {
            struct_ser.serialize_field("exitStatus", v)?;
        }
        if let Some(v) = self.error_status.as_ref() {
            struct_ser.serialize_field("errorStatus", v)?;
        }
        if !self.host.is_empty() {
            struct_ser.serialize_field("host", &self.host)?;
        }
        if self.domid != 0 {
            struct_ser.serialize_field("domid", &self.domid)?;
        }
        if let Some(v) = self.resource_status.as_ref() {
            struct_ser.serialize_field("resourceStatus", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.device_status.as_ref() {
            struct_ser.serialize_field("deviceStatus", v)?;
        }
        if let Some(v) = self.ready_at.as_ref() {
            struct_ser.serialize_field("readyAt", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "state",
            "network_status",
            "networkStatus",
            "exit_status",
            "exitStatus",
            "error_status",
            "errorStatus",
            "host",
            "domid",
            "resource_status",
            "resourceStatus",
            "created_at",
            "createdAt",
            "device_status",
            "deviceStatus",
            "ready_at",
            "readyAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
            NetworkStatus,
            ExitStatus,
            ErrorStatus,
            Host,
            Domid,
            ResourceStatus,
            CreatedAt,
            DeviceStatus,
            ReadyAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "state" => Ok(GeneratedField::State),
                            "networkStatus" | "network_status" => Ok(GeneratedField::NetworkStatus),
                            "exitStatus" | "exit_status" => Ok(GeneratedField::ExitStatus),
                            "errorStatus" | "error_status" => Ok(GeneratedField::ErrorStatus),
                            "host" => Ok(GeneratedField::Host),
                            "domid" => Ok(GeneratedField::Domid),
                            "resourceStatus" | "resource_status" => Ok(GeneratedField::ResourceStatus),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "deviceStatus" | "device_status" => Ok(GeneratedField::DeviceStatus),
                            "readyAt" | "ready_at" => Ok(GeneratedField::ReadyAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
                let mut network_status__ = None;
                let mut exit_status__ = None;
                let mut error_status__ = None;
                let mut host__ = None;
                let mut domid__ = None;
                let mut resource_status__ = None;
                let mut created_at__ = None;
                let mut device_status__ = None;
                let mut ready_at__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<ZoneState>()? as i32);
                        }
                        GeneratedField::NetworkStatus => {
                            if network_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("networkStatus"));
                            }
                            network_status__ = map_.next_value()?;
                        }
                        GeneratedField::ExitStatus => {
                            if exit_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exitStatus"));
                            }
                            exit_status__ = map_.next_value()?;
                        }
                        GeneratedField::ErrorStatus => {
                            if error_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorStatus"));
                            }
                            error_status__ = map_.next_value()?;
                        }
                        GeneratedField::Host => {
                            if host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            host__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Domid => {
                            if domid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domid"));
                            }
                            domid__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResourceStatus => {
                            if resource_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceStatus"));
                            }
                            resource_status__ = map_.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map_.next_value()?;
                        }
                        GeneratedField::DeviceStatus => {
                            if device_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceStatus"));
                            }
                            device_status__ = map_.next_value()?;
                        }
                        GeneratedField::ReadyAt => {
                            if ready_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readyAt"));
                            }
                            ready_at__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ZoneStatus {
                    state: state__.unwrap_or_default(),
                    network_status: network_status__,
                    exit_status: exit_status__,
                    error_status: error_status__,
                    host: host__.unwrap_or_default(),
                    domid: domid__.unwrap_or_default(),
                    resource_status: resource_status__,
                    created_at: created_at__,
                    device_status: device_status__,
                    ready_at: ready_at__,
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneVirtualizationBackend {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "ZONE_VIRTUALIZATION_BACKEND_UNKNOWN",
            Self::Pv => "ZONE_VIRTUALIZATION_BACKEND_PV",
            Self::Pvh => "ZONE_VIRTUALIZATION_BACKEND_PVH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ZoneVirtualizationBackend {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ZONE_VIRTUALIZATION_BACKEND_UNKNOWN",
            "ZONE_VIRTUALIZATION_BACKEND_PV",
            "ZONE_VIRTUALIZATION_BACKEND_PVH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneVirtualizationBackend;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ZONE_VIRTUALIZATION_BACKEND_UNKNOWN" => Ok(ZoneVirtualizationBackend::Unknown),
                    "ZONE_VIRTUALIZATION_BACKEND_PV" => Ok(ZoneVirtualizationBackend::Pv),
                    "ZONE_VIRTUALIZATION_BACKEND_PVH" => Ok(ZoneVirtualizationBackend::Pvh),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ZoneVirtualizationOptionsSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.backend != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("protect.control.v1.ZoneVirtualizationOptionsSpec", len)?;
        if self.backend != 0 {
            let v = ZoneVirtualizationBackend::try_from(self.backend)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.backend)))?;
            struct_ser.serialize_field("backend", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZoneVirtualizationOptionsSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "backend",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Backend,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "backend" => Ok(GeneratedField::Backend),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZoneVirtualizationOptionsSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct protect.control.v1.ZoneVirtualizationOptionsSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ZoneVirtualizationOptionsSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut backend__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Backend => {
                            if backend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backend"));
                            }
                            backend__ = Some(map_.next_value::<ZoneVirtualizationBackend>()? as i32);
                        }
                    }
                }
                Ok(ZoneVirtualizationOptionsSpec {
                    backend: backend__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("protect.control.v1.ZoneVirtualizationOptionsSpec", FIELDS, GeneratedVisitor)
    }
}
