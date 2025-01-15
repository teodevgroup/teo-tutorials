#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod helpers;
pub mod stdlib;

use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use chrono::{DateTime, Utc};
use teo::prelude::{
    teon, model, Model, Value, Result, Error, transaction, Request, ExtractFromRequest, ExtractFromPipelineCtx, request, pipeline, ExtractFromTransactionCtx, File, Arguments,
};
use std::marker::PhantomData;
use helpers::interface::{Interface, AsInterface, AsInterfaceRef, AsInterfaceVecRef};


/// ## Record scalar fields
///
/// This synthesized enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct RecordScalarFields {
    inner: String,
}

impl RecordScalarFields {
    /// ### Is Created at
    ///
    /// Returns true if value is created at
    pub fn is_created_at(&self) -> bool {
        self.inner.as_str() == "createdAt"
    }
    /// ### Created at
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn created_at() -> Self {
        Self { inner: "createdAt".to_owned() }
    }
    /// ### Is Id
    ///
    /// Returns true if value is id
    pub fn is_id(&self) -> bool {
        self.inner.as_str() == "id"
    }
    /// ### Id
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn id() -> Self {
        Self { inner: "id".to_owned() }
    }
    /// ### Is Updated at
    ///
    /// Returns true if value is updated at
    pub fn is_updated_at(&self) -> bool {
        self.inner.as_str() == "updatedAt"
    }
    /// ### Updated at
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn updated_at() -> Self {
        Self { inner: "updatedAt".to_owned() }
    }
    /// ### Is Value
    ///
    /// Returns true if value is value
    pub fn is_value(&self) -> bool {
        self.inner.as_str() == "value"
    }
    /// ### Value
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn value() -> Self {
        Self { inner: "value".to_owned() }
    }
}

impl From<RecordScalarFields> for Value {
    fn from(value: RecordScalarFields) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for RecordScalarFields {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "createdAt" => RecordScalarFields::created_at(),
                "id" => RecordScalarFields::id(),
                "updatedAt" => RecordScalarFields::updated_at(),
                "value" => RecordScalarFields::value(),
                _ => Err(Error::new("cannot convert value to RecordScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert value to RecordScalarFields"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &RecordScalarFields {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "createdAt" => unsafe { &*(enum_variant as *const str as *const Self) },
                "id" => unsafe { &*(enum_variant as *const str as *const Self) },
                "updatedAt" => unsafe { &*(enum_variant as *const str as *const Self) },
                "value" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &RecordScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &RecordScalarFields"))
        }
    }
}

impl AsInterface for RecordScalarFields {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "createdAt" => RecordScalarFields::created_at(),
                "id" => RecordScalarFields::id(),
                "updatedAt" => RecordScalarFields::updated_at(),
                "value" => RecordScalarFields::value(),
                _ => Err(Error::new("cannot convert value to RecordScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert value to RecordScalarFields"))
        }
    }
}

impl AsInterfaceRef for RecordScalarFields {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "createdAt" => unsafe { &*(enum_variant as *const str as *const Self) },
                "id" => unsafe { &*(enum_variant as *const str as *const Self) },
                "updatedAt" => unsafe { &*(enum_variant as *const str as *const Self) },
                "value" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &RecordScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &RecordScalarFields"))
        }
    }
}
/// ## Record serializable scalar fields
///
/// This synthesized enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct RecordSerializableScalarFields {
    inner: String,
}

impl RecordSerializableScalarFields {
    /// ### Is Created at
    ///
    /// Returns true if value is created at
    pub fn is_created_at(&self) -> bool {
        self.inner.as_str() == "createdAt"
    }
    /// ### Created at
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn created_at() -> Self {
        Self { inner: "createdAt".to_owned() }
    }
    /// ### Is Id
    ///
    /// Returns true if value is id
    pub fn is_id(&self) -> bool {
        self.inner.as_str() == "id"
    }
    /// ### Id
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn id() -> Self {
        Self { inner: "id".to_owned() }
    }
    /// ### Is Updated at
    ///
    /// Returns true if value is updated at
    pub fn is_updated_at(&self) -> bool {
        self.inner.as_str() == "updatedAt"
    }
    /// ### Updated at
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn updated_at() -> Self {
        Self { inner: "updatedAt".to_owned() }
    }
    /// ### Is Value
    ///
    /// Returns true if value is value
    pub fn is_value(&self) -> bool {
        self.inner.as_str() == "value"
    }
    /// ### Value
    ///
    /// This synthesized enum member doesn't have a description.
    pub fn value() -> Self {
        Self { inner: "value".to_owned() }
    }
}

impl From<RecordSerializableScalarFields> for Value {
    fn from(value: RecordSerializableScalarFields) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for RecordSerializableScalarFields {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "createdAt" => RecordSerializableScalarFields::created_at(),
                "id" => RecordSerializableScalarFields::id(),
                "updatedAt" => RecordSerializableScalarFields::updated_at(),
                "value" => RecordSerializableScalarFields::value(),
                _ => Err(Error::new("cannot convert value to RecordSerializableScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert value to RecordSerializableScalarFields"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &RecordSerializableScalarFields {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "createdAt" => unsafe { &*(enum_variant as *const str as *const Self) },
                "id" => unsafe { &*(enum_variant as *const str as *const Self) },
                "updatedAt" => unsafe { &*(enum_variant as *const str as *const Self) },
                "value" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &RecordSerializableScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &RecordSerializableScalarFields"))
        }
    }
}

impl AsInterface for RecordSerializableScalarFields {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "createdAt" => RecordSerializableScalarFields::created_at(),
                "id" => RecordSerializableScalarFields::id(),
                "updatedAt" => RecordSerializableScalarFields::updated_at(),
                "value" => RecordSerializableScalarFields::value(),
                _ => Err(Error::new("cannot convert value to RecordSerializableScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert value to RecordSerializableScalarFields"))
        }
    }
}

impl AsInterfaceRef for RecordSerializableScalarFields {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                "createdAt" => unsafe { &*(enum_variant as *const str as *const Self) },
                "id" => unsafe { &*(enum_variant as *const str as *const Self) },
                "updatedAt" => unsafe { &*(enum_variant as *const str as *const Self) },
                "value" => unsafe { &*(enum_variant as *const str as *const Self) },
                _ => Err(Error::new("cannot convert &Value to &RecordSerializableScalarFields"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &RecordSerializableScalarFields"))
        }
    }
}
/// ## Record relations
///
/// This synthesized enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct RecordRelations {
    inner: String,
}

impl RecordRelations {
}

impl From<RecordRelations> for Value {
    fn from(value: RecordRelations) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for RecordRelations {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to RecordRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to RecordRelations"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &RecordRelations {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &RecordRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &RecordRelations"))
        }
    }
}

impl AsInterface for RecordRelations {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to RecordRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to RecordRelations"))
        }
    }
}

impl AsInterfaceRef for RecordRelations {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &RecordRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &RecordRelations"))
        }
    }
}
/// ## Record direct relations
///
/// This synthesized enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct RecordDirectRelations {
    inner: String,
}

impl RecordDirectRelations {
}

impl From<RecordDirectRelations> for Value {
    fn from(value: RecordDirectRelations) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for RecordDirectRelations {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to RecordDirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to RecordDirectRelations"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &RecordDirectRelations {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &RecordDirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &RecordDirectRelations"))
        }
    }
}

impl AsInterface for RecordDirectRelations {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to RecordDirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to RecordDirectRelations"))
        }
    }
}

impl AsInterfaceRef for RecordDirectRelations {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &RecordDirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &RecordDirectRelations"))
        }
    }
}
/// ## Record indirect relations
///
/// This synthesized enum doesn't have a description.
#[repr(transparent)]
#[derive(PartialEq, Clone, Debug)]
pub struct RecordIndirectRelations {
    inner: String,
}

impl RecordIndirectRelations {
}

impl From<RecordIndirectRelations> for Value {
    fn from(value: RecordIndirectRelations) -> Value {
        Value::String(value.inner.clone())
    }
}

impl TryFrom<Value> for RecordIndirectRelations {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to RecordIndirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to RecordIndirectRelations"))
        }
    }
}

impl<'a> TryFrom<&'a Value> for &RecordIndirectRelations {

    type Error = Error;

    fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &RecordIndirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &RecordIndirectRelations"))
        }
    }
}

impl AsInterface for RecordIndirectRelations {
    fn from_value(value: Value) -> Result<Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert value to RecordIndirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert value to RecordIndirectRelations"))
        }
    }
}

impl AsInterfaceRef for RecordIndirectRelations {
    fn from_value_ref(value: &Value) -> Result<&Self> {
        if let Some(enum_variant) = value.as_str() {
            Ok(match enum_variant {
                _ => Err(Error::new("cannot convert &Value to &RecordIndirectRelations"))?
            })
        } else {
            Err(Error::new("cannot convert &Value to &RecordIndirectRelations"))
        }
    }
}

/// ## Record
///
/// This model doesn't have a description.
pub struct RecordModel {
    ctx: model::Ctx,
}

impl RecordModel {
    /// Find many record objects.
    pub async fn find_many_objects(&self, query: impl Borrow<Value>) -> Result<Vec<Record>> {
        Ok(self.ctx.find_many(query.borrow()).await?)
    }

    /// Find a unique record object.
    pub async fn find_unique_object(&self, query: impl Borrow<Value>) -> Result<Option<Record>> {
        Ok(self.ctx.find_unique(query.borrow()).await?)
    }

    /// Find a record object.
    pub async fn find_first_object(&self, query: impl Borrow<Value>) -> Result<Option<Record>> {
        Ok(self.ctx.find_first(query.borrow()).await?)
    }

    /// Create a new record object.
    pub async fn create_object(&self, values: impl Borrow<Value>) -> Result<Record> {
        Ok(self.ctx.create_object::<Record>(values.borrow()).await?)
    }

    /// Create an empty record object.
    pub async fn create_default_object(&self) -> Result<Record> {
        Ok(self.ctx.create_object::<Record>(teon!({}).borrow()).await?)
    }

    /// Count objects on record.
    pub async fn count_objects(&self, query: impl Borrow<Value>) -> Result<usize> {
        Ok(self.ctx.count_objects(query.borrow()).await?)
    }

    /// Count fields on record.
    pub async fn count_fields(&self, query: impl Borrow<Value>) -> Result<RecordCountAggregateResult> {
        Ok(RecordCountAggregateResult::from_value(self.ctx.count_fields(query.borrow()).await?)?)
    }

    /// Aggregate on record.
    pub async fn aggregate(&self, query: impl Borrow<Value>) -> Result<RecordAggregateResult> {
        Ok(RecordAggregateResult::from_value(self.ctx.aggregate(query.borrow()).await?)?)
    }

    /// Group by on record.
    pub async fn group_by(&self, query: impl Borrow<Value>) -> Result<Vec<RecordAggregateResult>> {
        let values: Vec<Value> = self.ctx.group_by(query.borrow()).await?;
        let mut result = vec![];
        for value in values.into_iter() {
            result.push(RecordAggregateResult::from_value(value)?);
        }
        Ok(result)
    }

    
    /// Run a custom SQL clause.
    pub async fn sql<T, E>(&self, sql: &str) -> Result<Vec<T>> where T: TryFrom<Value, Error=E>, Error: From<E> {
        self.ctx.sql(sql).await
    }
    
}

#[derive(Clone)]
pub struct Record {
    inner: model::Object,
}

impl Record {

    /// Whether this record is new.
    pub fn is_new(&self) -> bool {
        self.inner.is_new()
    }

    /// Whether this record is modified.
    pub fn is_modified(&self) -> bool {
        self.inner.is_modified()
    }

    /// Set new values to a record. Validations and transformations are
    /// triggered.
    pub async fn set(&self, values: impl AsRef<Value>) -> Result<()> {
        self.inner.set_teon(values.as_ref()).await
    }

    /// Update with new values to a record. Validations and transformations are
    /// not triggered.
    pub async fn update(&self, values: impl AsRef<Value>) -> Result<()> {
        self.inner.update_teon(values.as_ref()).await
    }

    /// Save this record.
    pub async fn save(&self) -> Result<()> {
        self.inner.save().await
    }

    /// Delete this record.
    pub async fn delete(&self) -> Result<()> {
        self.inner.delete().await
    }

    /// Convert this record object to teon.
    pub async fn to_teon(&self) -> Result<Value> {
        self.inner.to_teon().await
    }
    /// ## Id
    ///
    /// This field doesn't have a description.
    pub fn id(&self) -> Result<i32> {
        self.inner.get("id")
    }

    /// ## Id
    ///
    /// This field doesn't have a description.
    pub fn set_id(&self, new_value: i32) -> Result<()> {
        self.inner.set("id", new_value)
    }
    /// ## Value
    ///
    /// This field doesn't have a description.
    pub fn value(&self) -> Result<Option<i32>> {
        self.inner.get("value")
    }

    /// ## Value
    ///
    /// This field doesn't have a description.
    pub fn set_value(&self, new_value: Option<i32>) -> Result<()> {
        self.inner.set("value", new_value)
    }
    /// ## Created at
    ///
    /// This field doesn't have a description.
    pub fn created_at(&self) -> Result<DateTime<Utc>> {
        self.inner.get("createdAt")
    }

    /// ## Created at
    ///
    /// This field doesn't have a description.
    pub fn set_created_at(&self, new_value: DateTime<Utc>) -> Result<()> {
        self.inner.set("createdAt", new_value)
    }
    /// ## Updated at
    ///
    /// This field doesn't have a description.
    pub fn updated_at(&self) -> Result<DateTime<Utc>> {
        self.inner.get("updatedAt")
    }

    /// ## Updated at
    ///
    /// This field doesn't have a description.
    pub fn set_updated_at(&self, new_value: DateTime<Utc>) -> Result<()> {
        self.inner.set("updatedAt", new_value)
    }
}

impl From<Record> for model::Object {
    fn from(value: Record) -> Self {
        value.inner.clone()
    }
}

impl From<model::Object> for Record {
    fn from(value: model::Object) -> Self {
        Self { inner: value }
    }
}

impl From<Record> for Value {
    fn from(value: Record) -> Self {
        Value::ModelObject(value.inner.clone())
    }
}

impl AsInterface for Record {
    fn from_value(value: Value) -> Result<Self> {
        let model_object: model::Object = value.try_into()?;
        Ok(Self { inner: model_object })
    }
}

impl TryFrom<Value> for Record {

    type Error = Error;

    fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
        let model_object: model::Object = value.try_into()?;
        Ok(Self { inner: model_object })
    }
}

impl Debug for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl ExtractFromPipelineCtx for Record {
    fn extract(_: &Arguments, ctx: &pipeline::Ctx) -> Self {
        Record {
            inner: ctx.object().clone(),
        }
    }
}

pub trait AlterCreatedAtInputTrait: Interface {
    /// ## Id
    ///
    /// This interface field doesn't have a description.
    fn id(&self) -> &i32 {
        i32::from_value_ref(self.inner().get("id").unwrap()).unwrap()
    }
    /// ## Id
    ///
    /// This interface field doesn't have a description.
    fn set_id(&mut self, new_value: i32) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into()).unwrap();
    }
    /// ## Created at
    ///
    /// This interface field doesn't have a description.
    fn created_at(&self) -> &DateTime<Utc> {
        DateTime::<Utc>::from_value_ref(self.inner().get("createdAt").unwrap()).unwrap()
    }
    /// ## Created at
    ///
    /// This interface field doesn't have a description.
    fn set_created_at(&mut self, new_value: DateTime<Utc>) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("createdAt".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct AlterCreatedAtInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for AlterCreatedAtInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &AlterCreatedAtInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for AlterCreatedAtInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl AlterCreatedAtInputTrait for AlterCreatedAtInput { }

impl AsInterface for AlterCreatedAtInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<AlterCreatedAtInput> for Value {
    fn from(value: AlterCreatedAtInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for AlterCreatedAtInput {

    fn from_value_ref(value: &Value) -> Result<&AlterCreatedAtInput> {
        Ok(unsafe {
            &*(value as *const Value as *const AlterCreatedAtInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for AlterCreatedAtInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a AlterCreatedAtInput {
    fn extract(request: &'a Request) -> Self {
        AlterCreatedAtInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait UploadInputTrait: Interface {
    /// ## File
    ///
    /// This interface field doesn't have a description.
    fn file(&self) -> &File {
        File::from_value_ref(self.inner().get("file").unwrap()).unwrap()
    }
    /// ## File
    ///
    /// This interface field doesn't have a description.
    fn set_file(&mut self, new_value: File) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("file".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct UploadInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for UploadInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &UploadInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for UploadInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl UploadInputTrait for UploadInput { }

impl AsInterface for UploadInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<UploadInput> for Value {
    fn from(value: UploadInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for UploadInput {

    fn from_value_ref(value: &Value) -> Result<&UploadInput> {
        Ok(unsafe {
            &*(value as *const Value as *const UploadInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for UploadInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a UploadInput {
    fn extract(request: &'a Request) -> Self {
        UploadInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait UploadOutputTrait: Interface {
    /// ## Path
    ///
    /// This interface field doesn't have a description.
    fn path(&self) -> &String {
        String::from_value_ref(self.inner().get("path").unwrap()).unwrap()
    }
    /// ## Path
    ///
    /// This interface field doesn't have a description.
    fn set_path(&mut self, new_value: String) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("path".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct UploadOutput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for UploadOutput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &UploadOutput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for UploadOutput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl UploadOutputTrait for UploadOutput { }

impl AsInterface for UploadOutput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<UploadOutput> for Value {
    fn from(value: UploadOutput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for UploadOutput {

    fn from_value_ref(value: &Value) -> Result<&UploadOutput> {
        Ok(unsafe {
            &*(value as *const Value as *const UploadOutput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for UploadOutput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a UploadOutput {
    fn extract(request: &'a Request) -> Self {
        UploadOutput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordSelectTrait: Interface {
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn created_at(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("createdAt")?).unwrap())
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn set_created_at(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("createdAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("createdAt");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn updated_at(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("updatedAt")?).unwrap())
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn set_updated_at(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("updatedAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("updatedAt");
            },
        }
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn value(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("value")?).unwrap())
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn set_value(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("value".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("value");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordSelect {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordSelect {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordSelect {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordSelect {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordSelectTrait for RecordSelect { }

impl AsInterface for RecordSelect {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordSelect> for Value {
    fn from(value: RecordSelect) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordSelect {

    fn from_value_ref(value: &Value) -> Result<&RecordSelect> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordSelect)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordSelect {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordSelect {
    fn extract(request: &'a Request) -> Self {
        RecordSelect::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordIncludeTrait: Interface {
}

#[repr(transparent)]
pub struct RecordInclude {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordInclude {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordInclude {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordInclude {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordIncludeTrait for RecordInclude { }

impl AsInterface for RecordInclude {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordInclude> for Value {
    fn from(value: RecordInclude) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordInclude {

    fn from_value_ref(value: &Value) -> Result<&RecordInclude> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordInclude)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordInclude {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordInclude {
    fn extract(request: &'a Request) -> Self {
        RecordInclude::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordWhereInputTrait: Interface {
    /// ## And
    ///
    /// This synthesized field doesn't have a description.
    fn and(&self) -> Option<Vec<&RecordWhereInput>> {
        Some(Vec::<&RecordWhereInput>::from_value_ref_vec(self.inner().get("AND")?).unwrap())
    }
    /// ## And
    ///
    /// This synthesized field doesn't have a description.
    fn set_and(&mut self, new_value: Option<Vec<RecordWhereInput>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("AND".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("AND");
            },
        }
    }
    /// ## Not
    ///
    /// This synthesized field doesn't have a description.
    fn not(&self) -> Option<&RecordWhereInput> {
        Some(RecordWhereInput::from_value_ref(self.inner().get("NOT")?).unwrap())
    }
    /// ## Not
    ///
    /// This synthesized field doesn't have a description.
    fn set_not(&mut self, new_value: Option<RecordWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("NOT".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("NOT");
            },
        }
    }
    /// ## Or
    ///
    /// This synthesized field doesn't have a description.
    fn or(&self) -> Option<Vec<&RecordWhereInput>> {
        Some(Vec::<&RecordWhereInput>::from_value_ref_vec(self.inner().get("OR")?).unwrap())
    }
    /// ## Or
    ///
    /// This synthesized field doesn't have a description.
    fn set_or(&mut self, new_value: Option<Vec<RecordWhereInput>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("OR".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("OR");
            },
        }
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn created_at(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("createdAt")?).unwrap())
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn set_created_at(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("createdAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("createdAt");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn updated_at(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("updatedAt")?).unwrap())
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn set_updated_at(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("updatedAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("updatedAt");
            },
        }
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn value(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("value")?).unwrap())
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn set_value(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("value".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("value");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordWhereInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordWhereInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordWhereInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordWhereInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordWhereInputTrait for RecordWhereInput { }

impl AsInterface for RecordWhereInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordWhereInput> for Value {
    fn from(value: RecordWhereInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordWhereInput {

    fn from_value_ref(value: &Value) -> Result<&RecordWhereInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordWhereInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordWhereInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordWhereInput {
    fn extract(request: &'a Request) -> Self {
        RecordWhereInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordWhereUniqueInputTrait: Interface {
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> &i32 {
        i32::from_value_ref(self.inner().get("id").unwrap()).unwrap()
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: i32) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct RecordWhereUniqueInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordWhereUniqueInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordWhereUniqueInputTrait for RecordWhereUniqueInput { }

impl AsInterface for RecordWhereUniqueInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordWhereUniqueInput> for Value {
    fn from(value: RecordWhereUniqueInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordWhereUniqueInput {

    fn from_value_ref(value: &Value) -> Result<&RecordWhereUniqueInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordWhereUniqueInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordWhereUniqueInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordWhereUniqueInput {
    fn extract(request: &'a Request) -> Self {
        RecordWhereUniqueInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordScalarWhereWithAggregatesInputTrait: Interface {
    /// ## And
    ///
    /// This synthesized field doesn't have a description.
    fn and(&self) -> Option<Vec<&RecordWhereInput>> {
        Some(Vec::<&RecordWhereInput>::from_value_ref_vec(self.inner().get("AND")?).unwrap())
    }
    /// ## And
    ///
    /// This synthesized field doesn't have a description.
    fn set_and(&mut self, new_value: Option<Vec<RecordWhereInput>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("AND".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("AND");
            },
        }
    }
    /// ## Not
    ///
    /// This synthesized field doesn't have a description.
    fn not(&self) -> Option<&RecordWhereInput> {
        Some(RecordWhereInput::from_value_ref(self.inner().get("NOT")?).unwrap())
    }
    /// ## Not
    ///
    /// This synthesized field doesn't have a description.
    fn set_not(&mut self, new_value: Option<RecordWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("NOT".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("NOT");
            },
        }
    }
    /// ## Or
    ///
    /// This synthesized field doesn't have a description.
    fn or(&self) -> Option<Vec<&RecordWhereInput>> {
        Some(Vec::<&RecordWhereInput>::from_value_ref_vec(self.inner().get("OR")?).unwrap())
    }
    /// ## Or
    ///
    /// This synthesized field doesn't have a description.
    fn set_or(&mut self, new_value: Option<Vec<RecordWhereInput>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("OR".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("OR");
            },
        }
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn created_at(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("createdAt")?).unwrap())
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn set_created_at(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("createdAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("createdAt");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn updated_at(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("updatedAt")?).unwrap())
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn set_updated_at(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("updatedAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("updatedAt");
            },
        }
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn value(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("value")?).unwrap())
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn set_value(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("value".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("value");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordScalarWhereWithAggregatesInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordScalarWhereWithAggregatesInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordScalarWhereWithAggregatesInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordScalarWhereWithAggregatesInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordScalarWhereWithAggregatesInputTrait for RecordScalarWhereWithAggregatesInput { }

impl AsInterface for RecordScalarWhereWithAggregatesInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordScalarWhereWithAggregatesInput> for Value {
    fn from(value: RecordScalarWhereWithAggregatesInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordScalarWhereWithAggregatesInput {

    fn from_value_ref(value: &Value) -> Result<&RecordScalarWhereWithAggregatesInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordScalarWhereWithAggregatesInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordScalarWhereWithAggregatesInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordScalarWhereWithAggregatesInput {
    fn extract(request: &'a Request) -> Self {
        RecordScalarWhereWithAggregatesInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordRelationFilterTrait: Interface {
    /// ## Is
    ///
    /// This synthesized field doesn't have a description.
    fn is(&self) -> Option<&RecordWhereInput> {
        Some(RecordWhereInput::from_value_ref(self.inner().get("is")?).unwrap())
    }
    /// ## Is
    ///
    /// This synthesized field doesn't have a description.
    fn set_is(&mut self, new_value: Option<RecordWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("is".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("is");
            },
        }
    }
    /// ## Is Not
    ///
    /// This synthesized field doesn't have a description.
    fn is_not(&self) -> Option<&RecordWhereInput> {
        Some(RecordWhereInput::from_value_ref(self.inner().get("isNot")?).unwrap())
    }
    /// ## Is Not
    ///
    /// This synthesized field doesn't have a description.
    fn set_is_not(&mut self, new_value: Option<RecordWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("isNot".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("isNot");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordRelationFilter {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordRelationFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordRelationFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordRelationFilter {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordRelationFilterTrait for RecordRelationFilter { }

impl AsInterface for RecordRelationFilter {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordRelationFilter> for Value {
    fn from(value: RecordRelationFilter) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordRelationFilter {

    fn from_value_ref(value: &Value) -> Result<&RecordRelationFilter> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordRelationFilter)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordRelationFilter {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordRelationFilter {
    fn extract(request: &'a Request) -> Self {
        RecordRelationFilter::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordListRelationFilterTrait: Interface {
    /// ## Every
    ///
    /// This synthesized field doesn't have a description.
    fn every(&self) -> Option<&RecordWhereInput> {
        Some(RecordWhereInput::from_value_ref(self.inner().get("every")?).unwrap())
    }
    /// ## Every
    ///
    /// This synthesized field doesn't have a description.
    fn set_every(&mut self, new_value: Option<RecordWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("every".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("every");
            },
        }
    }
    /// ## None
    ///
    /// This synthesized field doesn't have a description.
    fn none(&self) -> Option<&RecordWhereInput> {
        Some(RecordWhereInput::from_value_ref(self.inner().get("none")?).unwrap())
    }
    /// ## None
    ///
    /// This synthesized field doesn't have a description.
    fn set_none(&mut self, new_value: Option<RecordWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("none".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("none");
            },
        }
    }
    /// ## Some
    ///
    /// This synthesized field doesn't have a description.
    fn some(&self) -> Option<&RecordWhereInput> {
        Some(RecordWhereInput::from_value_ref(self.inner().get("some")?).unwrap())
    }
    /// ## Some
    ///
    /// This synthesized field doesn't have a description.
    fn set_some(&mut self, new_value: Option<RecordWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("some".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("some");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordListRelationFilter {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordListRelationFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordListRelationFilter {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordListRelationFilter {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordListRelationFilterTrait for RecordListRelationFilter { }

impl AsInterface for RecordListRelationFilter {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordListRelationFilter> for Value {
    fn from(value: RecordListRelationFilter) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordListRelationFilter {

    fn from_value_ref(value: &Value) -> Result<&RecordListRelationFilter> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordListRelationFilter)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordListRelationFilter {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordListRelationFilter {
    fn extract(request: &'a Request) -> Self {
        RecordListRelationFilter::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordOrderByInputTrait: Interface {
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn created_at(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("createdAt")?).unwrap())
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn set_created_at(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("createdAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("createdAt");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn updated_at(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("updatedAt")?).unwrap())
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn set_updated_at(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("updatedAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("updatedAt");
            },
        }
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn value(&self) -> Option<&stdlib::Sort> {
        Some(stdlib::Sort::from_value_ref(self.inner().get("value")?).unwrap())
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn set_value(&mut self, new_value: Option<stdlib::Sort>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("value".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("value");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordOrderByInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordOrderByInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordOrderByInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordOrderByInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordOrderByInputTrait for RecordOrderByInput { }

impl AsInterface for RecordOrderByInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordOrderByInput> for Value {
    fn from(value: RecordOrderByInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordOrderByInput {

    fn from_value_ref(value: &Value) -> Result<&RecordOrderByInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordOrderByInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordOrderByInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordOrderByInput {
    fn extract(request: &'a Request) -> Self {
        RecordOrderByInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordCountAggregateInputTypeTrait: Interface {
    /// ## All
    ///
    /// This synthesized field doesn't have a description.
    fn all(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("_all")?).unwrap())
    }
    /// ## All
    ///
    /// This synthesized field doesn't have a description.
    fn set_all(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_all".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_all");
            },
        }
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn created_at(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("createdAt")?).unwrap())
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn set_created_at(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("createdAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("createdAt");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn updated_at(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("updatedAt")?).unwrap())
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn set_updated_at(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("updatedAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("updatedAt");
            },
        }
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn value(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("value")?).unwrap())
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn set_value(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("value".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("value");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordCountAggregateInputType {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordCountAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordCountAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordCountAggregateInputType {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordCountAggregateInputTypeTrait for RecordCountAggregateInputType { }

impl AsInterface for RecordCountAggregateInputType {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordCountAggregateInputType> for Value {
    fn from(value: RecordCountAggregateInputType) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordCountAggregateInputType {

    fn from_value_ref(value: &Value) -> Result<&RecordCountAggregateInputType> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordCountAggregateInputType)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordCountAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordCountAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        RecordCountAggregateInputType::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordSumAggregateInputTypeTrait: Interface {
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordSumAggregateInputType {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordSumAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordSumAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordSumAggregateInputType {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordSumAggregateInputTypeTrait for RecordSumAggregateInputType { }

impl AsInterface for RecordSumAggregateInputType {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordSumAggregateInputType> for Value {
    fn from(value: RecordSumAggregateInputType) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordSumAggregateInputType {

    fn from_value_ref(value: &Value) -> Result<&RecordSumAggregateInputType> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordSumAggregateInputType)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordSumAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordSumAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        RecordSumAggregateInputType::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordAvgAggregateInputTypeTrait: Interface {
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordAvgAggregateInputType {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordAvgAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordAvgAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordAvgAggregateInputType {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordAvgAggregateInputTypeTrait for RecordAvgAggregateInputType { }

impl AsInterface for RecordAvgAggregateInputType {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordAvgAggregateInputType> for Value {
    fn from(value: RecordAvgAggregateInputType) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordAvgAggregateInputType {

    fn from_value_ref(value: &Value) -> Result<&RecordAvgAggregateInputType> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordAvgAggregateInputType)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordAvgAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordAvgAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        RecordAvgAggregateInputType::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordMinAggregateInputTypeTrait: Interface {
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn created_at(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("createdAt")?).unwrap())
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn set_created_at(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("createdAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("createdAt");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn updated_at(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("updatedAt")?).unwrap())
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn set_updated_at(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("updatedAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("updatedAt");
            },
        }
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn value(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("value")?).unwrap())
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn set_value(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("value".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("value");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordMinAggregateInputType {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordMinAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordMinAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordMinAggregateInputType {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordMinAggregateInputTypeTrait for RecordMinAggregateInputType { }

impl AsInterface for RecordMinAggregateInputType {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordMinAggregateInputType> for Value {
    fn from(value: RecordMinAggregateInputType) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordMinAggregateInputType {

    fn from_value_ref(value: &Value) -> Result<&RecordMinAggregateInputType> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordMinAggregateInputType)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordMinAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordMinAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        RecordMinAggregateInputType::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordMaxAggregateInputTypeTrait: Interface {
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn created_at(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("createdAt")?).unwrap())
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn set_created_at(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("createdAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("createdAt");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn updated_at(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("updatedAt")?).unwrap())
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn set_updated_at(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("updatedAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("updatedAt");
            },
        }
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn value(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("value")?).unwrap())
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn set_value(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("value".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("value");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordMaxAggregateInputType {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordMaxAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordMaxAggregateInputType {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordMaxAggregateInputType {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordMaxAggregateInputTypeTrait for RecordMaxAggregateInputType { }

impl AsInterface for RecordMaxAggregateInputType {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordMaxAggregateInputType> for Value {
    fn from(value: RecordMaxAggregateInputType) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordMaxAggregateInputType {

    fn from_value_ref(value: &Value) -> Result<&RecordMaxAggregateInputType> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordMaxAggregateInputType)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordMaxAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordMaxAggregateInputType {
    fn extract(request: &'a Request) -> Self {
        RecordMaxAggregateInputType::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordCreateInputTrait: Interface {
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn value(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("value")?).unwrap())
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn set_value(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("value".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("value");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordCreateInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordCreateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordCreateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordCreateInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordCreateInputTrait for RecordCreateInput { }

impl AsInterface for RecordCreateInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordCreateInput> for Value {
    fn from(value: RecordCreateInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordCreateInput {

    fn from_value_ref(value: &Value) -> Result<&RecordCreateInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordCreateInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordCreateInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordCreateInput {
    fn extract(request: &'a Request) -> Self {
        RecordCreateInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordUpdateInputTrait: Interface {
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn value(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("value")?).unwrap())
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn set_value(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("value".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("value");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordUpdateInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordUpdateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordUpdateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordUpdateInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordUpdateInputTrait for RecordUpdateInput { }

impl AsInterface for RecordUpdateInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordUpdateInput> for Value {
    fn from(value: RecordUpdateInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordUpdateInput {

    fn from_value_ref(value: &Value) -> Result<&RecordUpdateInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordUpdateInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordUpdateInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordUpdateInput {
    fn extract(request: &'a Request) -> Self {
        RecordUpdateInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordCreateNestedOneInputTrait: Interface {
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn connect(&self) -> Option<&RecordWhereUniqueInput> {
        Some(RecordWhereUniqueInput::from_value_ref(self.inner().get("connect")?).unwrap())
    }
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect(&mut self, new_value: Option<RecordWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connect".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connect");
            },
        }
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn connect_or_create(&self) -> Option<&RecordConnectOrCreateInput> {
        Some(RecordConnectOrCreateInput::from_value_ref(self.inner().get("connectOrCreate")?).unwrap())
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect_or_create(&mut self, new_value: Option<RecordConnectOrCreateInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connectOrCreate".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connectOrCreate");
            },
        }
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> Option<&RecordCreateInput> {
        Some(RecordCreateInput::from_value_ref(self.inner().get("create")?).unwrap())
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: Option<RecordCreateInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("create");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordCreateNestedOneInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordCreateNestedOneInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordCreateNestedOneInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordCreateNestedOneInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordCreateNestedOneInputTrait for RecordCreateNestedOneInput { }

impl AsInterface for RecordCreateNestedOneInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordCreateNestedOneInput> for Value {
    fn from(value: RecordCreateNestedOneInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordCreateNestedOneInput {

    fn from_value_ref(value: &Value) -> Result<&RecordCreateNestedOneInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordCreateNestedOneInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordCreateNestedOneInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordCreateNestedOneInput {
    fn extract(request: &'a Request) -> Self {
        RecordCreateNestedOneInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordCreateNestedManyInputTrait: Interface {
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn connect(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("connect")?).unwrap())
    }
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connect".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connect");
            },
        }
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn connect_or_create(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("connectOrCreate")?).unwrap())
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect_or_create(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connectOrCreate".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connectOrCreate");
            },
        }
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("create")?).unwrap())
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("create");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordCreateNestedManyInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordCreateNestedManyInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordCreateNestedManyInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordCreateNestedManyInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordCreateNestedManyInputTrait for RecordCreateNestedManyInput { }

impl AsInterface for RecordCreateNestedManyInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordCreateNestedManyInput> for Value {
    fn from(value: RecordCreateNestedManyInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordCreateNestedManyInput {

    fn from_value_ref(value: &Value) -> Result<&RecordCreateNestedManyInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordCreateNestedManyInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordCreateNestedManyInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordCreateNestedManyInput {
    fn extract(request: &'a Request) -> Self {
        RecordCreateNestedManyInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordUpdateNestedOneInputTrait: Interface {
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn connect(&self) -> Option<&RecordWhereUniqueInput> {
        Some(RecordWhereUniqueInput::from_value_ref(self.inner().get("connect")?).unwrap())
    }
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect(&mut self, new_value: Option<RecordWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connect".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connect");
            },
        }
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn connect_or_create(&self) -> Option<&RecordConnectOrCreateInput> {
        Some(RecordConnectOrCreateInput::from_value_ref(self.inner().get("connectOrCreate")?).unwrap())
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect_or_create(&mut self, new_value: Option<RecordConnectOrCreateInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connectOrCreate".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connectOrCreate");
            },
        }
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> Option<&RecordCreateInput> {
        Some(RecordCreateInput::from_value_ref(self.inner().get("create")?).unwrap())
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: Option<RecordCreateInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("create");
            },
        }
    }
    /// ## Delete
    ///
    /// This synthesized field doesn't have a description.
    fn delete(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("delete")?).unwrap())
    }
    /// ## Delete
    ///
    /// This synthesized field doesn't have a description.
    fn set_delete(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("delete".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("delete");
            },
        }
    }
    /// ## Disconnect
    ///
    /// This synthesized field doesn't have a description.
    fn disconnect(&self) -> Option<&bool> {
        Some(bool::from_value_ref(self.inner().get("disconnect")?).unwrap())
    }
    /// ## Disconnect
    ///
    /// This synthesized field doesn't have a description.
    fn set_disconnect(&mut self, new_value: Option<bool>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("disconnect".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("disconnect");
            },
        }
    }
    /// ## Set
    ///
    /// This synthesized field doesn't have a description.
    fn set(&self) -> Option<&RecordWhereUniqueInput> {
        Some(RecordWhereUniqueInput::from_value_ref(self.inner().get("set")?).unwrap())
    }
    /// ## Set
    ///
    /// This synthesized field doesn't have a description.
    fn set_set(&mut self, new_value: Option<RecordWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("set".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("set");
            },
        }
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> Option<&RecordUpdateInput> {
        Some(RecordUpdateInput::from_value_ref(self.inner().get("update")?).unwrap())
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: Option<RecordUpdateInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("update");
            },
        }
    }
    /// ## Upsert
    ///
    /// This synthesized field doesn't have a description.
    fn upsert(&self) -> Option<&RecordUpsertWithWhereUniqueInput> {
        Some(RecordUpsertWithWhereUniqueInput::from_value_ref(self.inner().get("upsert")?).unwrap())
    }
    /// ## Upsert
    ///
    /// This synthesized field doesn't have a description.
    fn set_upsert(&mut self, new_value: Option<RecordUpsertWithWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("upsert".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("upsert");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordUpdateNestedOneInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordUpdateNestedOneInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordUpdateNestedOneInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordUpdateNestedOneInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordUpdateNestedOneInputTrait for RecordUpdateNestedOneInput { }

impl AsInterface for RecordUpdateNestedOneInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordUpdateNestedOneInput> for Value {
    fn from(value: RecordUpdateNestedOneInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordUpdateNestedOneInput {

    fn from_value_ref(value: &Value) -> Result<&RecordUpdateNestedOneInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordUpdateNestedOneInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordUpdateNestedOneInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordUpdateNestedOneInput {
    fn extract(request: &'a Request) -> Self {
        RecordUpdateNestedOneInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordUpdateNestedManyInputTrait: Interface {
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn connect(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("connect")?).unwrap())
    }
    /// ## Connect
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connect".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connect");
            },
        }
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn connect_or_create(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("connectOrCreate")?).unwrap())
    }
    /// ## Connect Or Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_connect_or_create(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("connectOrCreate".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("connectOrCreate");
            },
        }
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("create")?).unwrap())
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("create");
            },
        }
    }
    /// ## Delete
    ///
    /// This synthesized field doesn't have a description.
    fn delete(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("delete")?).unwrap())
    }
    /// ## Delete
    ///
    /// This synthesized field doesn't have a description.
    fn set_delete(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("delete".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("delete");
            },
        }
    }
    /// ## Delete Many
    ///
    /// This synthesized field doesn't have a description.
    fn delete_many(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("deleteMany")?).unwrap())
    }
    /// ## Delete Many
    ///
    /// This synthesized field doesn't have a description.
    fn set_delete_many(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("deleteMany".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("deleteMany");
            },
        }
    }
    /// ## Disconnect
    ///
    /// This synthesized field doesn't have a description.
    fn disconnect(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("disconnect")?).unwrap())
    }
    /// ## Disconnect
    ///
    /// This synthesized field doesn't have a description.
    fn set_disconnect(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("disconnect".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("disconnect");
            },
        }
    }
    /// ## Set
    ///
    /// This synthesized field doesn't have a description.
    fn set(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("set")?).unwrap())
    }
    /// ## Set
    ///
    /// This synthesized field doesn't have a description.
    fn set_set(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("set".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("set");
            },
        }
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("update")?).unwrap())
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("update");
            },
        }
    }
    /// ## Update Many
    ///
    /// This synthesized field doesn't have a description.
    fn update_many(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("updateMany")?).unwrap())
    }
    /// ## Update Many
    ///
    /// This synthesized field doesn't have a description.
    fn set_update_many(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("updateMany".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("updateMany");
            },
        }
    }
    /// ## Upsert
    ///
    /// This synthesized field doesn't have a description.
    fn upsert(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("upsert")?).unwrap())
    }
    /// ## Upsert
    ///
    /// This synthesized field doesn't have a description.
    fn set_upsert(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("upsert".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("upsert");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordUpdateNestedManyInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordUpdateNestedManyInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordUpdateNestedManyInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordUpdateNestedManyInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordUpdateNestedManyInputTrait for RecordUpdateNestedManyInput { }

impl AsInterface for RecordUpdateNestedManyInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordUpdateNestedManyInput> for Value {
    fn from(value: RecordUpdateNestedManyInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordUpdateNestedManyInput {

    fn from_value_ref(value: &Value) -> Result<&RecordUpdateNestedManyInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordUpdateNestedManyInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordUpdateNestedManyInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordUpdateNestedManyInput {
    fn extract(request: &'a Request) -> Self {
        RecordUpdateNestedManyInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordConnectOrCreateInputTrait: Interface {
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> &RecordCreateInput {
        RecordCreateInput::from_value_ref(self.inner().get("create").unwrap()).unwrap()
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: RecordCreateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &RecordWhereUniqueInput {
        RecordWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: RecordWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct RecordConnectOrCreateInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordConnectOrCreateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordConnectOrCreateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordConnectOrCreateInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordConnectOrCreateInputTrait for RecordConnectOrCreateInput { }

impl AsInterface for RecordConnectOrCreateInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordConnectOrCreateInput> for Value {
    fn from(value: RecordConnectOrCreateInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordConnectOrCreateInput {

    fn from_value_ref(value: &Value) -> Result<&RecordConnectOrCreateInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordConnectOrCreateInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordConnectOrCreateInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordConnectOrCreateInput {
    fn extract(request: &'a Request) -> Self {
        RecordConnectOrCreateInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordUpdateWithWhereUniqueInputTrait: Interface {
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> &RecordUpdateInput {
        RecordUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: RecordUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &RecordWhereUniqueInput {
        RecordWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: RecordWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct RecordUpdateWithWhereUniqueInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordUpdateWithWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordUpdateWithWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordUpdateWithWhereUniqueInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordUpdateWithWhereUniqueInputTrait for RecordUpdateWithWhereUniqueInput { }

impl AsInterface for RecordUpdateWithWhereUniqueInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordUpdateWithWhereUniqueInput> for Value {
    fn from(value: RecordUpdateWithWhereUniqueInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordUpdateWithWhereUniqueInput {

    fn from_value_ref(value: &Value) -> Result<&RecordUpdateWithWhereUniqueInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordUpdateWithWhereUniqueInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordUpdateWithWhereUniqueInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordUpdateWithWhereUniqueInput {
    fn extract(request: &'a Request) -> Self {
        RecordUpdateWithWhereUniqueInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordUpsertWithWhereUniqueInputTrait: Interface {
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> &RecordCreateInput {
        RecordCreateInput::from_value_ref(self.inner().get("create").unwrap()).unwrap()
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: RecordCreateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into()).unwrap();
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> &RecordUpdateInput {
        RecordUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: RecordUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &RecordWhereUniqueInput {
        RecordWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: RecordWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct RecordUpsertWithWhereUniqueInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordUpsertWithWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordUpsertWithWhereUniqueInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordUpsertWithWhereUniqueInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordUpsertWithWhereUniqueInputTrait for RecordUpsertWithWhereUniqueInput { }

impl AsInterface for RecordUpsertWithWhereUniqueInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordUpsertWithWhereUniqueInput> for Value {
    fn from(value: RecordUpsertWithWhereUniqueInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordUpsertWithWhereUniqueInput {

    fn from_value_ref(value: &Value) -> Result<&RecordUpsertWithWhereUniqueInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordUpsertWithWhereUniqueInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordUpsertWithWhereUniqueInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordUpsertWithWhereUniqueInput {
    fn extract(request: &'a Request) -> Self {
        RecordUpsertWithWhereUniqueInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordUpdateManyWithWhereInputTrait: Interface {
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> &RecordUpdateInput {
        RecordUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: RecordUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &RecordWhereInput {
        RecordWhereInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: RecordWhereInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct RecordUpdateManyWithWhereInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordUpdateManyWithWhereInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordUpdateManyWithWhereInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordUpdateManyWithWhereInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordUpdateManyWithWhereInputTrait for RecordUpdateManyWithWhereInput { }

impl AsInterface for RecordUpdateManyWithWhereInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordUpdateManyWithWhereInput> for Value {
    fn from(value: RecordUpdateManyWithWhereInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordUpdateManyWithWhereInput {

    fn from_value_ref(value: &Value) -> Result<&RecordUpdateManyWithWhereInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordUpdateManyWithWhereInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordUpdateManyWithWhereInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordUpdateManyWithWhereInput {
    fn extract(request: &'a Request) -> Self {
        RecordUpdateManyWithWhereInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordResultTrait: Interface {
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn created_at(&self) -> &DateTime<Utc> {
        DateTime::<Utc>::from_value_ref(self.inner().get("createdAt").unwrap()).unwrap()
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn set_created_at(&mut self, new_value: DateTime<Utc>) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("createdAt".to_owned(), new_value.into()).unwrap();
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> &i32 {
        i32::from_value_ref(self.inner().get("id").unwrap()).unwrap()
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: i32) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into()).unwrap();
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn updated_at(&self) -> &DateTime<Utc> {
        DateTime::<Utc>::from_value_ref(self.inner().get("updatedAt").unwrap()).unwrap()
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn set_updated_at(&mut self, new_value: DateTime<Utc>) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("updatedAt".to_owned(), new_value.into()).unwrap();
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn value(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("value")?).unwrap())
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn set_value(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("value".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("value");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordResultTrait for RecordResult { }

impl AsInterface for RecordResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordResult> for Value {
    fn from(value: RecordResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordResult {

    fn from_value_ref(value: &Value) -> Result<&RecordResult> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordResult {
    fn extract(request: &'a Request) -> Self {
        RecordResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordCountAggregateResultTrait: Interface {
    /// ## All
    ///
    /// This synthesized field doesn't have a description.
    fn all(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("_all")?).unwrap())
    }
    /// ## All
    ///
    /// This synthesized field doesn't have a description.
    fn set_all(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_all".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_all");
            },
        }
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn created_at(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("createdAt")?).unwrap())
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn set_created_at(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("createdAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("createdAt");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn updated_at(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("updatedAt")?).unwrap())
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn set_updated_at(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("updatedAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("updatedAt");
            },
        }
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn value(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("value")?).unwrap())
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn set_value(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("value".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("value");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordCountAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordCountAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordCountAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordCountAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordCountAggregateResultTrait for RecordCountAggregateResult { }

impl AsInterface for RecordCountAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordCountAggregateResult> for Value {
    fn from(value: RecordCountAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordCountAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&RecordCountAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordCountAggregateResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordCountAggregateResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordCountAggregateResult {
    fn extract(request: &'a Request) -> Self {
        RecordCountAggregateResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordSumAggregateResultTrait: Interface {
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordSumAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordSumAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordSumAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordSumAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordSumAggregateResultTrait for RecordSumAggregateResult { }

impl AsInterface for RecordSumAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordSumAggregateResult> for Value {
    fn from(value: RecordSumAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordSumAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&RecordSumAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordSumAggregateResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordSumAggregateResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordSumAggregateResult {
    fn extract(request: &'a Request) -> Self {
        RecordSumAggregateResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordAvgAggregateResultTrait: Interface {
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&f64> {
        Some(f64::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<f64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordAvgAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordAvgAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordAvgAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordAvgAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordAvgAggregateResultTrait for RecordAvgAggregateResult { }

impl AsInterface for RecordAvgAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordAvgAggregateResult> for Value {
    fn from(value: RecordAvgAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordAvgAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&RecordAvgAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordAvgAggregateResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordAvgAggregateResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordAvgAggregateResult {
    fn extract(request: &'a Request) -> Self {
        RecordAvgAggregateResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordMinAggregateResultTrait: Interface {
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn created_at(&self) -> Option<&DateTime<Utc>> {
        Some(DateTime::<Utc>::from_value_ref(self.inner().get("createdAt")?).unwrap())
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn set_created_at(&mut self, new_value: Option<DateTime<Utc>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("createdAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("createdAt");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn updated_at(&self) -> Option<&DateTime<Utc>> {
        Some(DateTime::<Utc>::from_value_ref(self.inner().get("updatedAt")?).unwrap())
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn set_updated_at(&mut self, new_value: Option<DateTime<Utc>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("updatedAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("updatedAt");
            },
        }
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn value(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("value")?).unwrap())
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn set_value(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("value".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("value");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordMinAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordMinAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordMinAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordMinAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordMinAggregateResultTrait for RecordMinAggregateResult { }

impl AsInterface for RecordMinAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordMinAggregateResult> for Value {
    fn from(value: RecordMinAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordMinAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&RecordMinAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordMinAggregateResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordMinAggregateResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordMinAggregateResult {
    fn extract(request: &'a Request) -> Self {
        RecordMinAggregateResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordMaxAggregateResultTrait: Interface {
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn created_at(&self) -> Option<&DateTime<Utc>> {
        Some(DateTime::<Utc>::from_value_ref(self.inner().get("createdAt")?).unwrap())
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn set_created_at(&mut self, new_value: Option<DateTime<Utc>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("createdAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("createdAt");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn updated_at(&self) -> Option<&DateTime<Utc>> {
        Some(DateTime::<Utc>::from_value_ref(self.inner().get("updatedAt")?).unwrap())
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn set_updated_at(&mut self, new_value: Option<DateTime<Utc>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("updatedAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("updatedAt");
            },
        }
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn value(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("value")?).unwrap())
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn set_value(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("value".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("value");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordMaxAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordMaxAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordMaxAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordMaxAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordMaxAggregateResultTrait for RecordMaxAggregateResult { }

impl AsInterface for RecordMaxAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordMaxAggregateResult> for Value {
    fn from(value: RecordMaxAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordMaxAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&RecordMaxAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordMaxAggregateResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordMaxAggregateResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordMaxAggregateResult {
    fn extract(request: &'a Request) -> Self {
        RecordMaxAggregateResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordAggregateResultTrait: Interface {
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn avg(&self) -> Option<&RecordAvgAggregateResult> {
        Some(RecordAvgAggregateResult::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<RecordAvgAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_avg".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_avg");
            },
        }
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn count(&self) -> Option<&RecordCountAggregateResult> {
        Some(RecordCountAggregateResult::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn set_count(&mut self, new_value: Option<RecordCountAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn max(&self) -> Option<&RecordMaxAggregateResult> {
        Some(RecordMaxAggregateResult::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn set_max(&mut self, new_value: Option<RecordMaxAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn min(&self) -> Option<&RecordMinAggregateResult> {
        Some(RecordMinAggregateResult::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn set_min(&mut self, new_value: Option<RecordMinAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn sum(&self) -> Option<&RecordSumAggregateResult> {
        Some(RecordSumAggregateResult::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<RecordSumAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_sum".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_sum");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordAggregateResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordAggregateResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordAggregateResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordAggregateResultTrait for RecordAggregateResult { }

impl AsInterface for RecordAggregateResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordAggregateResult> for Value {
    fn from(value: RecordAggregateResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordAggregateResult {

    fn from_value_ref(value: &Value) -> Result<&RecordAggregateResult> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordAggregateResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordAggregateResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordAggregateResult {
    fn extract(request: &'a Request) -> Self {
        RecordAggregateResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordGroupByResultTrait: Interface {
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn avg(&self) -> Option<&RecordAvgAggregateResult> {
        Some(RecordAvgAggregateResult::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<RecordAvgAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_avg".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_avg");
            },
        }
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn count(&self) -> Option<&RecordCountAggregateResult> {
        Some(RecordCountAggregateResult::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn set_count(&mut self, new_value: Option<RecordCountAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn max(&self) -> Option<&RecordMaxAggregateResult> {
        Some(RecordMaxAggregateResult::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn set_max(&mut self, new_value: Option<RecordMaxAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn min(&self) -> Option<&RecordMinAggregateResult> {
        Some(RecordMinAggregateResult::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn set_min(&mut self, new_value: Option<RecordMinAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn sum(&self) -> Option<&RecordSumAggregateResult> {
        Some(RecordSumAggregateResult::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<RecordSumAggregateResult>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_sum".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_sum");
            },
        }
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn created_at(&self) -> Option<&DateTime<Utc>> {
        Some(DateTime::<Utc>::from_value_ref(self.inner().get("createdAt")?).unwrap())
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn set_created_at(&mut self, new_value: Option<DateTime<Utc>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("createdAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("createdAt");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn updated_at(&self) -> Option<&DateTime<Utc>> {
        Some(DateTime::<Utc>::from_value_ref(self.inner().get("updatedAt")?).unwrap())
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn set_updated_at(&mut self, new_value: Option<DateTime<Utc>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("updatedAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("updatedAt");
            },
        }
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn value(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("value")?).unwrap())
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn set_value(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("value".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("value");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordGroupByResult {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordGroupByResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordGroupByResult {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordGroupByResult {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordGroupByResultTrait for RecordGroupByResult { }

impl AsInterface for RecordGroupByResult {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordGroupByResult> for Value {
    fn from(value: RecordGroupByResult) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordGroupByResult {

    fn from_value_ref(value: &Value) -> Result<&RecordGroupByResult> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordGroupByResult)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordGroupByResult {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordGroupByResult {
    fn extract(request: &'a Request) -> Self {
        RecordGroupByResult::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&RecordInclude> {
        Some(RecordInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<RecordInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&RecordSelect> {
        Some(RecordSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<RecordSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordArgsTrait for RecordArgs { }

impl AsInterface for RecordArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordArgs> for Value {
    fn from(value: RecordArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordArgs {
    fn extract(request: &'a Request) -> Self {
        RecordArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordFindManyArgsTrait: Interface {
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn cursor(&self) -> Option<&RecordWhereUniqueInput> {
        Some(RecordWhereUniqueInput::from_value_ref(self.inner().get("cursor")?).unwrap())
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn set_cursor(&mut self, new_value: Option<RecordWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("cursor".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("cursor");
            },
        }
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn distinct(&self) -> Option<&RecordSerializableScalarFields> {
        Some(RecordSerializableScalarFields::from_value_ref(self.inner().get("distinct")?).unwrap())
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn set_distinct(&mut self, new_value: Option<RecordSerializableScalarFields>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("distinct".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("distinct");
            },
        }
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&RecordInclude> {
        Some(RecordInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<RecordInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn order_by(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("orderBy")?).unwrap())
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn set_order_by(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("orderBy".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("orderBy");
            },
        }
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn page_number(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageNumber")?).unwrap())
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_number(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageNumber".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageNumber");
            },
        }
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn page_size(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageSize")?).unwrap())
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_size(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageSize".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageSize");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&RecordSelect> {
        Some(RecordSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<RecordSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn skip(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("skip")?).unwrap())
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn set_skip(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("skip".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("skip");
            },
        }
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn take(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("take")?).unwrap())
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn set_take(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("take".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("take");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> Option<&RecordWhereInput> {
        Some(RecordWhereInput::from_value_ref(self.inner().get("where")?).unwrap())
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: Option<RecordWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("where");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordFindManyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordFindManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordFindManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordFindManyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordFindManyArgsTrait for RecordFindManyArgs { }

impl AsInterface for RecordFindManyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordFindManyArgs> for Value {
    fn from(value: RecordFindManyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordFindManyArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordFindManyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordFindManyArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordFindManyArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordFindManyArgs {
    fn extract(request: &'a Request) -> Self {
        RecordFindManyArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordFindFirstArgsTrait: Interface {
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn cursor(&self) -> Option<&RecordWhereUniqueInput> {
        Some(RecordWhereUniqueInput::from_value_ref(self.inner().get("cursor")?).unwrap())
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn set_cursor(&mut self, new_value: Option<RecordWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("cursor".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("cursor");
            },
        }
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn distinct(&self) -> Option<&RecordSerializableScalarFields> {
        Some(RecordSerializableScalarFields::from_value_ref(self.inner().get("distinct")?).unwrap())
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn set_distinct(&mut self, new_value: Option<RecordSerializableScalarFields>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("distinct".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("distinct");
            },
        }
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&RecordInclude> {
        Some(RecordInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<RecordInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn order_by(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("orderBy")?).unwrap())
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn set_order_by(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("orderBy".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("orderBy");
            },
        }
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn page_number(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageNumber")?).unwrap())
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_number(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageNumber".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageNumber");
            },
        }
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn page_size(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageSize")?).unwrap())
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_size(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageSize".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageSize");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&RecordSelect> {
        Some(RecordSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<RecordSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn skip(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("skip")?).unwrap())
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn set_skip(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("skip".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("skip");
            },
        }
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn take(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("take")?).unwrap())
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn set_take(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("take".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("take");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> Option<&RecordWhereInput> {
        Some(RecordWhereInput::from_value_ref(self.inner().get("where")?).unwrap())
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: Option<RecordWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("where");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordFindFirstArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordFindFirstArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordFindFirstArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordFindFirstArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordFindFirstArgsTrait for RecordFindFirstArgs { }

impl AsInterface for RecordFindFirstArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordFindFirstArgs> for Value {
    fn from(value: RecordFindFirstArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordFindFirstArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordFindFirstArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordFindFirstArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordFindFirstArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordFindFirstArgs {
    fn extract(request: &'a Request) -> Self {
        RecordFindFirstArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordFindUniqueArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&RecordInclude> {
        Some(RecordInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<RecordInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&RecordSelect> {
        Some(RecordSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<RecordSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &RecordWhereUniqueInput {
        RecordWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: RecordWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct RecordFindUniqueArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordFindUniqueArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordFindUniqueArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordFindUniqueArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordFindUniqueArgsTrait for RecordFindUniqueArgs { }

impl AsInterface for RecordFindUniqueArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordFindUniqueArgs> for Value {
    fn from(value: RecordFindUniqueArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordFindUniqueArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordFindUniqueArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordFindUniqueArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordFindUniqueArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordFindUniqueArgs {
    fn extract(request: &'a Request) -> Self {
        RecordFindUniqueArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordCreateArgsTrait: Interface {
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> &RecordCreateInput {
        RecordCreateInput::from_value_ref(self.inner().get("create").unwrap()).unwrap()
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: RecordCreateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&RecordInclude> {
        Some(RecordInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<RecordInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&RecordSelect> {
        Some(RecordSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<RecordSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordCreateArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordCreateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordCreateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordCreateArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordCreateArgsTrait for RecordCreateArgs { }

impl AsInterface for RecordCreateArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordCreateArgs> for Value {
    fn from(value: RecordCreateArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordCreateArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordCreateArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordCreateArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordCreateArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordCreateArgs {
    fn extract(request: &'a Request) -> Self {
        RecordCreateArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordUpdateArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&RecordInclude> {
        Some(RecordInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<RecordInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&RecordSelect> {
        Some(RecordSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<RecordSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> &RecordUpdateInput {
        RecordUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: RecordUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &RecordWhereUniqueInput {
        RecordWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: RecordWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct RecordUpdateArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordUpdateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordUpdateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordUpdateArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordUpdateArgsTrait for RecordUpdateArgs { }

impl AsInterface for RecordUpdateArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordUpdateArgs> for Value {
    fn from(value: RecordUpdateArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordUpdateArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordUpdateArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordUpdateArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordUpdateArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordUpdateArgs {
    fn extract(request: &'a Request) -> Self {
        RecordUpdateArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordUpsertArgsTrait: Interface {
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> &RecordCreateInput {
        RecordCreateInput::from_value_ref(self.inner().get("create").unwrap()).unwrap()
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: RecordCreateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&RecordInclude> {
        Some(RecordInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<RecordInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&RecordSelect> {
        Some(RecordSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<RecordSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> &RecordUpdateInput {
        RecordUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: RecordUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &RecordWhereUniqueInput {
        RecordWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: RecordWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct RecordUpsertArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordUpsertArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordUpsertArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordUpsertArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordUpsertArgsTrait for RecordUpsertArgs { }

impl AsInterface for RecordUpsertArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordUpsertArgs> for Value {
    fn from(value: RecordUpsertArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordUpsertArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordUpsertArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordUpsertArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordUpsertArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordUpsertArgs {
    fn extract(request: &'a Request) -> Self {
        RecordUpsertArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordCopyArgsTrait: Interface {
    /// ## Copy
    ///
    /// This synthesized field doesn't have a description.
    fn copy(&self) -> &RecordUpdateInput {
        RecordUpdateInput::from_value_ref(self.inner().get("copy").unwrap()).unwrap()
    }
    /// ## Copy
    ///
    /// This synthesized field doesn't have a description.
    fn set_copy(&mut self, new_value: RecordUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("copy".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&RecordInclude> {
        Some(RecordInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<RecordInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&RecordSelect> {
        Some(RecordSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<RecordSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &RecordWhereUniqueInput {
        RecordWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: RecordWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct RecordCopyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordCopyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordCopyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordCopyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordCopyArgsTrait for RecordCopyArgs { }

impl AsInterface for RecordCopyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordCopyArgs> for Value {
    fn from(value: RecordCopyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordCopyArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordCopyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordCopyArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordCopyArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordCopyArgs {
    fn extract(request: &'a Request) -> Self {
        RecordCopyArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordDeleteArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&RecordInclude> {
        Some(RecordInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<RecordInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&RecordSelect> {
        Some(RecordSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<RecordSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &RecordWhereUniqueInput {
        RecordWhereUniqueInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: RecordWhereUniqueInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct RecordDeleteArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordDeleteArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordDeleteArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordDeleteArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordDeleteArgsTrait for RecordDeleteArgs { }

impl AsInterface for RecordDeleteArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordDeleteArgs> for Value {
    fn from(value: RecordDeleteArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordDeleteArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordDeleteArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordDeleteArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordDeleteArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordDeleteArgs {
    fn extract(request: &'a Request) -> Self {
        RecordDeleteArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordCreateManyArgsTrait: Interface {
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn create(&self) -> &Value {
        Value::from_value_ref(self.inner().get("create").unwrap()).unwrap()
    }
    /// ## Create
    ///
    /// This synthesized field doesn't have a description.
    fn set_create(&mut self, new_value: Value) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("create".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&RecordInclude> {
        Some(RecordInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<RecordInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&RecordSelect> {
        Some(RecordSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<RecordSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordCreateManyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordCreateManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordCreateManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordCreateManyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordCreateManyArgsTrait for RecordCreateManyArgs { }

impl AsInterface for RecordCreateManyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordCreateManyArgs> for Value {
    fn from(value: RecordCreateManyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordCreateManyArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordCreateManyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordCreateManyArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordCreateManyArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordCreateManyArgs {
    fn extract(request: &'a Request) -> Self {
        RecordCreateManyArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordUpdateManyArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&RecordInclude> {
        Some(RecordInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<RecordInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&RecordSelect> {
        Some(RecordSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<RecordSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn update(&self) -> &RecordUpdateInput {
        RecordUpdateInput::from_value_ref(self.inner().get("update").unwrap()).unwrap()
    }
    /// ## Update
    ///
    /// This synthesized field doesn't have a description.
    fn set_update(&mut self, new_value: RecordUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("update".to_owned(), new_value.into()).unwrap();
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &RecordWhereInput {
        RecordWhereInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: RecordWhereInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct RecordUpdateManyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordUpdateManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordUpdateManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordUpdateManyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordUpdateManyArgsTrait for RecordUpdateManyArgs { }

impl AsInterface for RecordUpdateManyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordUpdateManyArgs> for Value {
    fn from(value: RecordUpdateManyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordUpdateManyArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordUpdateManyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordUpdateManyArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordUpdateManyArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordUpdateManyArgs {
    fn extract(request: &'a Request) -> Self {
        RecordUpdateManyArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordDeleteManyArgsTrait: Interface {
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&RecordInclude> {
        Some(RecordInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<RecordInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&RecordSelect> {
        Some(RecordSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<RecordSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &RecordWhereInput {
        RecordWhereInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: RecordWhereInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct RecordDeleteManyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordDeleteManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordDeleteManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordDeleteManyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordDeleteManyArgsTrait for RecordDeleteManyArgs { }

impl AsInterface for RecordDeleteManyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordDeleteManyArgs> for Value {
    fn from(value: RecordDeleteManyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordDeleteManyArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordDeleteManyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordDeleteManyArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordDeleteManyArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordDeleteManyArgs {
    fn extract(request: &'a Request) -> Self {
        RecordDeleteManyArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordCopyManyArgsTrait: Interface {
    /// ## Copy
    ///
    /// This synthesized field doesn't have a description.
    fn copy(&self) -> &RecordUpdateInput {
        RecordUpdateInput::from_value_ref(self.inner().get("copy").unwrap()).unwrap()
    }
    /// ## Copy
    ///
    /// This synthesized field doesn't have a description.
    fn set_copy(&mut self, new_value: RecordUpdateInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("copy".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&RecordInclude> {
        Some(RecordInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<RecordInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&RecordSelect> {
        Some(RecordSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<RecordSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> &RecordWhereInput {
        RecordWhereInput::from_value_ref(self.inner().get("where").unwrap()).unwrap()
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: RecordWhereInput) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into()).unwrap();
    }
}

#[repr(transparent)]
pub struct RecordCopyManyArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordCopyManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordCopyManyArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordCopyManyArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordCopyManyArgsTrait for RecordCopyManyArgs { }

impl AsInterface for RecordCopyManyArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordCopyManyArgs> for Value {
    fn from(value: RecordCopyManyArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordCopyManyArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordCopyManyArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordCopyManyArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordCopyManyArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordCopyManyArgs {
    fn extract(request: &'a Request) -> Self {
        RecordCopyManyArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordCountArgsTrait: Interface {
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn cursor(&self) -> Option<&RecordWhereUniqueInput> {
        Some(RecordWhereUniqueInput::from_value_ref(self.inner().get("cursor")?).unwrap())
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn set_cursor(&mut self, new_value: Option<RecordWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("cursor".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("cursor");
            },
        }
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn distinct(&self) -> Option<&RecordSerializableScalarFields> {
        Some(RecordSerializableScalarFields::from_value_ref(self.inner().get("distinct")?).unwrap())
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn set_distinct(&mut self, new_value: Option<RecordSerializableScalarFields>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("distinct".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("distinct");
            },
        }
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn order_by(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("orderBy")?).unwrap())
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn set_order_by(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("orderBy".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("orderBy");
            },
        }
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn page_number(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageNumber")?).unwrap())
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_number(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageNumber".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageNumber");
            },
        }
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn page_size(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageSize")?).unwrap())
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_size(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageSize".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageSize");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&RecordCountAggregateInputType> {
        Some(RecordCountAggregateInputType::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<RecordCountAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn skip(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("skip")?).unwrap())
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn set_skip(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("skip".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("skip");
            },
        }
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn take(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("take")?).unwrap())
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn set_take(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("take".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("take");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> Option<&RecordWhereInput> {
        Some(RecordWhereInput::from_value_ref(self.inner().get("where")?).unwrap())
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: Option<RecordWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("where");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordCountArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordCountArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordCountArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordCountArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordCountArgsTrait for RecordCountArgs { }

impl AsInterface for RecordCountArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordCountArgs> for Value {
    fn from(value: RecordCountArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordCountArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordCountArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordCountArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordCountArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordCountArgs {
    fn extract(request: &'a Request) -> Self {
        RecordCountArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordAggregateArgsTrait: Interface {
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn avg(&self) -> Option<&RecordAvgAggregateInputType> {
        Some(RecordAvgAggregateInputType::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<RecordAvgAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_avg".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_avg");
            },
        }
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn count(&self) -> Option<&RecordCountAggregateInputType> {
        Some(RecordCountAggregateInputType::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn set_count(&mut self, new_value: Option<RecordCountAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn max(&self) -> Option<&RecordMaxAggregateInputType> {
        Some(RecordMaxAggregateInputType::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn set_max(&mut self, new_value: Option<RecordMaxAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn min(&self) -> Option<&RecordMinAggregateInputType> {
        Some(RecordMinAggregateInputType::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn set_min(&mut self, new_value: Option<RecordMinAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn sum(&self) -> Option<&RecordSumAggregateInputType> {
        Some(RecordSumAggregateInputType::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<RecordSumAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_sum".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_sum");
            },
        }
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn cursor(&self) -> Option<&RecordWhereUniqueInput> {
        Some(RecordWhereUniqueInput::from_value_ref(self.inner().get("cursor")?).unwrap())
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn set_cursor(&mut self, new_value: Option<RecordWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("cursor".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("cursor");
            },
        }
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn distinct(&self) -> Option<&RecordSerializableScalarFields> {
        Some(RecordSerializableScalarFields::from_value_ref(self.inner().get("distinct")?).unwrap())
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn set_distinct(&mut self, new_value: Option<RecordSerializableScalarFields>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("distinct".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("distinct");
            },
        }
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn order_by(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("orderBy")?).unwrap())
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn set_order_by(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("orderBy".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("orderBy");
            },
        }
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn page_number(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageNumber")?).unwrap())
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_number(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageNumber".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageNumber");
            },
        }
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn page_size(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageSize")?).unwrap())
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_size(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageSize".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageSize");
            },
        }
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn skip(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("skip")?).unwrap())
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn set_skip(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("skip".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("skip");
            },
        }
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn take(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("take")?).unwrap())
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn set_take(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("take".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("take");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> Option<&RecordWhereInput> {
        Some(RecordWhereInput::from_value_ref(self.inner().get("where")?).unwrap())
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: Option<RecordWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("where");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordAggregateArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordAggregateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordAggregateArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordAggregateArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordAggregateArgsTrait for RecordAggregateArgs { }

impl AsInterface for RecordAggregateArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordAggregateArgs> for Value {
    fn from(value: RecordAggregateArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordAggregateArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordAggregateArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordAggregateArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordAggregateArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordAggregateArgs {
    fn extract(request: &'a Request) -> Self {
        RecordAggregateArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordGroupByArgsTrait: Interface {
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn avg(&self) -> Option<&RecordAvgAggregateInputType> {
        Some(RecordAvgAggregateInputType::from_value_ref(self.inner().get("_avg")?).unwrap())
    }
    /// ## Avg
    ///
    /// This synthesized field doesn't have a description.
    fn set_avg(&mut self, new_value: Option<RecordAvgAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_avg".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_avg");
            },
        }
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn count(&self) -> Option<&RecordCountAggregateInputType> {
        Some(RecordCountAggregateInputType::from_value_ref(self.inner().get("_count")?).unwrap())
    }
    /// ## Count
    ///
    /// This synthesized field doesn't have a description.
    fn set_count(&mut self, new_value: Option<RecordCountAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_count".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_count");
            },
        }
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn max(&self) -> Option<&RecordMaxAggregateInputType> {
        Some(RecordMaxAggregateInputType::from_value_ref(self.inner().get("_max")?).unwrap())
    }
    /// ## Max
    ///
    /// This synthesized field doesn't have a description.
    fn set_max(&mut self, new_value: Option<RecordMaxAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_max".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_max");
            },
        }
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn min(&self) -> Option<&RecordMinAggregateInputType> {
        Some(RecordMinAggregateInputType::from_value_ref(self.inner().get("_min")?).unwrap())
    }
    /// ## Min
    ///
    /// This synthesized field doesn't have a description.
    fn set_min(&mut self, new_value: Option<RecordMinAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_min".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_min");
            },
        }
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn sum(&self) -> Option<&RecordSumAggregateInputType> {
        Some(RecordSumAggregateInputType::from_value_ref(self.inner().get("_sum")?).unwrap())
    }
    /// ## Sum
    ///
    /// This synthesized field doesn't have a description.
    fn set_sum(&mut self, new_value: Option<RecordSumAggregateInputType>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("_sum".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("_sum");
            },
        }
    }
    /// ## By
    ///
    /// This synthesized field doesn't have a description.
    fn by(&self) -> &Value {
        Value::from_value_ref(self.inner().get("by").unwrap()).unwrap()
    }
    /// ## By
    ///
    /// This synthesized field doesn't have a description.
    fn set_by(&mut self, new_value: Value) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("by".to_owned(), new_value.into()).unwrap();
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn cursor(&self) -> Option<&RecordWhereUniqueInput> {
        Some(RecordWhereUniqueInput::from_value_ref(self.inner().get("cursor")?).unwrap())
    }
    /// ## Cursor
    ///
    /// This synthesized field doesn't have a description.
    fn set_cursor(&mut self, new_value: Option<RecordWhereUniqueInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("cursor".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("cursor");
            },
        }
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn distinct(&self) -> Option<&RecordSerializableScalarFields> {
        Some(RecordSerializableScalarFields::from_value_ref(self.inner().get("distinct")?).unwrap())
    }
    /// ## Distinct
    ///
    /// This synthesized field doesn't have a description.
    fn set_distinct(&mut self, new_value: Option<RecordSerializableScalarFields>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("distinct".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("distinct");
            },
        }
    }
    /// ## Having
    ///
    /// This synthesized field doesn't have a description.
    fn having(&self) -> Option<&RecordScalarWhereWithAggregatesInput> {
        Some(RecordScalarWhereWithAggregatesInput::from_value_ref(self.inner().get("having")?).unwrap())
    }
    /// ## Having
    ///
    /// This synthesized field doesn't have a description.
    fn set_having(&mut self, new_value: Option<RecordScalarWhereWithAggregatesInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("having".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("having");
            },
        }
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn order_by(&self) -> Option<&Value> {
        Some(Value::from_value_ref(self.inner().get("orderBy")?).unwrap())
    }
    /// ## Order By
    ///
    /// This synthesized field doesn't have a description.
    fn set_order_by(&mut self, new_value: Option<Value>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("orderBy".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("orderBy");
            },
        }
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn page_number(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageNumber")?).unwrap())
    }
    /// ## Page Number
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_number(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageNumber".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageNumber");
            },
        }
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn page_size(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("pageSize")?).unwrap())
    }
    /// ## Page Size
    ///
    /// This synthesized field doesn't have a description.
    fn set_page_size(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("pageSize".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("pageSize");
            },
        }
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn skip(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("skip")?).unwrap())
    }
    /// ## Skip
    ///
    /// This synthesized field doesn't have a description.
    fn set_skip(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("skip".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("skip");
            },
        }
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn take(&self) -> Option<&i64> {
        Some(i64::from_value_ref(self.inner().get("take")?).unwrap())
    }
    /// ## Take
    ///
    /// This synthesized field doesn't have a description.
    fn set_take(&mut self, new_value: Option<i64>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("take".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("take");
            },
        }
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn r#where(&self) -> Option<&RecordWhereInput> {
        Some(RecordWhereInput::from_value_ref(self.inner().get("where")?).unwrap())
    }
    /// ## Where
    ///
    /// This synthesized field doesn't have a description.
    fn set_where(&mut self, new_value: Option<RecordWhereInput>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("where".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("where");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordGroupByArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordGroupByArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordGroupByArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordGroupByArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordGroupByArgsTrait for RecordGroupByArgs { }

impl AsInterface for RecordGroupByArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordGroupByArgs> for Value {
    fn from(value: RecordGroupByArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordGroupByArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordGroupByArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordGroupByArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordGroupByArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordGroupByArgs {
    fn extract(request: &'a Request) -> Self {
        RecordGroupByArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordScalarUpdateInputTrait: Interface {
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn created_at(&self) -> Option<&DateTime<Utc>> {
        Some(DateTime::<Utc>::from_value_ref(self.inner().get("createdAt")?).unwrap())
    }
    /// ## Created At
    ///
    /// This synthesized field doesn't have a description.
    fn set_created_at(&mut self, new_value: Option<DateTime<Utc>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("createdAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("createdAt");
            },
        }
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn id(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("id")?).unwrap())
    }
    /// ## Id
    ///
    /// This synthesized field doesn't have a description.
    fn set_id(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("id".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("id");
            },
        }
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn updated_at(&self) -> Option<&DateTime<Utc>> {
        Some(DateTime::<Utc>::from_value_ref(self.inner().get("updatedAt")?).unwrap())
    }
    /// ## Updated At
    ///
    /// This synthesized field doesn't have a description.
    fn set_updated_at(&mut self, new_value: Option<DateTime<Utc>>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("updatedAt".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("updatedAt");
            },
        }
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn value(&self) -> Option<&i32> {
        Some(i32::from_value_ref(self.inner().get("value")?).unwrap())
    }
    /// ## Value
    ///
    /// This synthesized field doesn't have a description.
    fn set_value(&mut self, new_value: Option<i32>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("value".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("value");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordScalarUpdateInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordScalarUpdateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordScalarUpdateInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordScalarUpdateInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordScalarUpdateInputTrait for RecordScalarUpdateInput { }

impl AsInterface for RecordScalarUpdateInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordScalarUpdateInput> for Value {
    fn from(value: RecordScalarUpdateInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordScalarUpdateInput {

    fn from_value_ref(value: &Value) -> Result<&RecordScalarUpdateInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordScalarUpdateInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordScalarUpdateInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordScalarUpdateInput {
    fn extract(request: &'a Request) -> Self {
        RecordScalarUpdateInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordSignInCheckerIdsTrait: Interface {
}

#[repr(transparent)]
pub struct RecordSignInCheckerIds {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordSignInCheckerIds {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordSignInCheckerIds {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordSignInCheckerIds {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordSignInCheckerIdsTrait for RecordSignInCheckerIds { }

impl AsInterface for RecordSignInCheckerIds {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordSignInCheckerIds> for Value {
    fn from(value: RecordSignInCheckerIds) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordSignInCheckerIds {

    fn from_value_ref(value: &Value) -> Result<&RecordSignInCheckerIds> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordSignInCheckerIds)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordSignInCheckerIds {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordSignInCheckerIds {
    fn extract(request: &'a Request) -> Self {
        RecordSignInCheckerIds::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordSignInCheckerCompanionsTrait: Interface {
}

#[repr(transparent)]
pub struct RecordSignInCheckerCompanions {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordSignInCheckerCompanions {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordSignInCheckerCompanions {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordSignInCheckerCompanions {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordSignInCheckerCompanionsTrait for RecordSignInCheckerCompanions { }

impl AsInterface for RecordSignInCheckerCompanions {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordSignInCheckerCompanions> for Value {
    fn from(value: RecordSignInCheckerCompanions) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordSignInCheckerCompanions {

    fn from_value_ref(value: &Value) -> Result<&RecordSignInCheckerCompanions> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordSignInCheckerCompanions)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordSignInCheckerCompanions {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordSignInCheckerCompanions {
    fn extract(request: &'a Request) -> Self {
        RecordSignInCheckerCompanions::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordSignInInputTrait: Interface {
    /// ## Credentials
    ///
    /// This synthesized field doesn't have a description.
    fn credentials(&self) -> &RecordSignInArgs {
        RecordSignInArgs::from_value_ref(self.inner().get("credentials").unwrap()).unwrap()
    }
    /// ## Credentials
    ///
    /// This synthesized field doesn't have a description.
    fn set_credentials(&mut self, new_value: RecordSignInArgs) {
    self.inner_mut().as_dictionary_mut().unwrap().insert("credentials".to_owned(), new_value.into()).unwrap();
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn include(&self) -> Option<&RecordInclude> {
        Some(RecordInclude::from_value_ref(self.inner().get("include")?).unwrap())
    }
    /// ## Include
    ///
    /// This synthesized field doesn't have a description.
    fn set_include(&mut self, new_value: Option<RecordInclude>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("include".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("include");
            },
        }
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn select(&self) -> Option<&RecordSelect> {
        Some(RecordSelect::from_value_ref(self.inner().get("select")?).unwrap())
    }
    /// ## Select
    ///
    /// This synthesized field doesn't have a description.
    fn set_select(&mut self, new_value: Option<RecordSelect>) {
    
        match new_value {
            Some(new_value) => {
                self.inner_mut().as_dictionary_mut().unwrap().insert("select".to_owned(), new_value.into());
            },
            None => {
                self.inner_mut().as_dictionary_mut().unwrap().shift_remove("select");
            },
        }
    }
}

#[repr(transparent)]
pub struct RecordSignInInput {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordSignInInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordSignInInput {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordSignInInput {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordSignInInputTrait for RecordSignInInput { }

impl AsInterface for RecordSignInInput {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordSignInInput> for Value {
    fn from(value: RecordSignInInput) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordSignInInput {

    fn from_value_ref(value: &Value) -> Result<&RecordSignInInput> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordSignInInput)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordSignInInput {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordSignInInput {
    fn extract(request: &'a Request) -> Self {
        RecordSignInInput::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

pub trait RecordSignInArgsTrait: Interface {
}

#[repr(transparent)]
pub struct RecordSignInArgs {
    inner: Value,
    phantom_data: PhantomData<()>,
}

impl Borrow<Value> for RecordSignInArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Borrow<Value> for &RecordSignInArgs {
    fn borrow(&self) -> &Value {
        self.inner()
    }
}

impl Interface for RecordSignInArgs {
    fn inner(&self) -> &Value {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Value {
        &mut self.inner
    }
}

impl RecordSignInArgsTrait for RecordSignInArgs { }

impl AsInterface for RecordSignInArgs {

    fn from_value(value: Value) -> Result<Self> {
        Ok(Self { inner: value, phantom_data: PhantomData::default() })
    }
}

impl From<RecordSignInArgs> for Value {
    fn from(value: RecordSignInArgs) -> Self {
        value.inner
    }
}

impl AsInterfaceRef for RecordSignInArgs {

    fn from_value_ref(value: &Value) -> Result<&RecordSignInArgs> {
        Ok(unsafe {
            &*(value as *const Value as *const RecordSignInArgs)
        })
    }
}

impl<'a> ExtractFromRequest<'a> for RecordSignInArgs {
    fn extract(request: &'a Request) -> Self {
        Self::from_value(request.body_value().unwrap().clone()).unwrap()
    }
}

impl<'a> ExtractFromRequest<'a> for &'a RecordSignInArgs {
    fn extract(request: &'a Request) -> Self {
        RecordSignInArgs::from_value_ref(request.body_value().unwrap()).unwrap()
    }
}

#[repr(transparent)]
pub struct EchoPathArguments {
    request: Request,
}

impl EchoPathArguments {

    pub fn data(&self) -> Result<&str> {
        Ok(self.request.captures()?.get("data").unwrap().as_str())
    }
}

impl<'a> ExtractFromRequest<'a> for EchoPathArguments {
    fn extract(request: &'a Request) -> Self {
        EchoPathArguments {
            request: request.clone(),
        }
    }
}

#[repr(transparent)]
pub struct StaticPathArguments {
    request: Request,
}

impl StaticPathArguments {

    pub fn path(&self) -> Result<&str> {
        Ok(self.request.captures()?.get("path").unwrap().as_str())
    }
}

impl<'a> ExtractFromRequest<'a> for StaticPathArguments {
    fn extract(request: &'a Request) -> Self {
        StaticPathArguments {
            request: request.clone(),
        }
    }
}

pub struct Teo {
    pub(crate) ctx: transaction::Ctx,
}

impl From<transaction::Ctx> for Teo {
    fn from(value: transaction::Ctx) -> Self {
        Self { ctx: value }
    }
}

impl Teo {

    pub async fn transaction<F, C, Fut, R>(&self, f: F) -> Result<R> where
        F: Fn(C) -> Fut,
        C: for <'a> From<&'a transaction::Ctx>,
        Fut: Future<Output = Result<R>> {
        Ok(self.ctx.run_transaction(f).await?)
    }
    
    pub fn record(&self) -> RecordModel {
        RecordModel { ctx: self.ctx.model_ctx_for_model_at_path(&vec!["Record".to_owned()]).unwrap() }
    }
}


impl ExtractFromTransactionCtx for Teo {
    fn extract(ctx: &transaction::Ctx) -> Self {
        Teo {
            ctx: ctx.clone(),
        }
    }
}

impl<'a> ExtractFromRequest<'a> for Teo {
    fn extract(request: &'a Request) -> Self {
        Teo {
            ctx: request.transaction_ctx().clone(),
        }
    }
}

impl ExtractFromPipelineCtx for Teo {
    fn extract(_: &Arguments, ctx: &pipeline::Ctx) -> Self {
        Teo {
            ctx: ctx.transaction_ctx().clone(),
        }
    }
}
