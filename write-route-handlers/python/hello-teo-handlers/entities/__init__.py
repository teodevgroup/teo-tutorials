from __future__ import annotations
from typing import Any, Optional, Literal, TypedDict, Generic, TypeVar, NotRequired, cast, TYPE_CHECKING
from re import Pattern
from datetime import date, datetime
from decimal import Decimal
from teo import ObjectId, Enumerable, File, Range, OptionVariant

from . import std


RecordScalarFields = Literal["createdAt", "id", "updatedAt", "value"]

RecordSerializableScalarFields = Literal["createdAt", "id", "updatedAt", "value"]

RecordRelations = Literal[None]

RecordDirectRelations = Literal[None]

RecordIndirectRelations = Literal[None]



# **Alter created at input**
#
# This interface doesn't have a description.
class AlterCreatedAtInput(TypedDict):


    # **Id**
    #
    # This interface field doesn't have a description.
    id: int

    # **Created at**
    #
    # This interface field doesn't have a description.
    createdAt: datetime


# **Upload input**
#
# This interface doesn't have a description.
class UploadInput(TypedDict):


    # **File**
    #
    # This interface field doesn't have a description.
    file: File


# **Upload output**
#
# This interface doesn't have a description.
class UploadOutput(TypedDict):


    # **Path**
    #
    # This interface field doesn't have a description.
    path: str


# **Record select**
#
# This synthesized interface doesn't have a description
class RecordSelect(TypedDict):


    # **Created At**
    #
    # This synthesized field doesn't have a description.
    createdAt: NotRequired[Optional[bool]]

    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[bool]]

    # **Updated At**
    #
    # This synthesized field doesn't have a description.
    updatedAt: NotRequired[Optional[bool]]

    # **Value**
    #
    # This synthesized field doesn't have a description.
    value: NotRequired[Optional[bool]]


# **Record include**
#
# This synthesized interface doesn't have a description
class RecordInclude(TypedDict):

    pass



# **Record where input**
#
# This synthesized interface doesn't have a description
class RecordWhereInput(TypedDict):


    # **And**
    #
    # This synthesized field doesn't have a description.
    AND: NotRequired[Optional[list[RecordWhereInput]]]

    # **Not**
    #
    # This synthesized field doesn't have a description.
    NOT: NotRequired[Optional[RecordWhereInput]]

    # **Or**
    #
    # This synthesized field doesn't have a description.
    OR: NotRequired[Optional[list[RecordWhereInput]]]

    # **Created At**
    #
    # This synthesized field doesn't have a description.
    createdAt: NotRequired[Optional[datetime | std.Filter[datetime]]]

    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[int | std.Filter[int]]]

    # **Updated At**
    #
    # This synthesized field doesn't have a description.
    updatedAt: NotRequired[Optional[datetime | std.Filter[datetime]]]

    # **Value**
    #
    # This synthesized field doesn't have a description.
    value: NotRequired[Optional[int | None | std.NullableFilter[int]]]


# **Record where unique input**
#
# This synthesized interface doesn't have a description
class RecordWhereUniqueInput(TypedDict):


    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: int


# **Record scalar where with aggregates input**
#
# This synthesized interface doesn't have a description
class RecordScalarWhereWithAggregatesInput(TypedDict):


    # **And**
    #
    # This synthesized field doesn't have a description.
    AND: NotRequired[Optional[list[RecordWhereInput]]]

    # **Not**
    #
    # This synthesized field doesn't have a description.
    NOT: NotRequired[Optional[RecordWhereInput]]

    # **Or**
    #
    # This synthesized field doesn't have a description.
    OR: NotRequired[Optional[list[RecordWhereInput]]]

    # **Created At**
    #
    # This synthesized field doesn't have a description.
    createdAt: NotRequired[Optional[datetime | std.AggregatesFilter[datetime]]]

    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[int | std.IntNumberWithAggregatesFilter[int]]]

    # **Updated At**
    #
    # This synthesized field doesn't have a description.
    updatedAt: NotRequired[Optional[datetime | std.AggregatesFilter[datetime]]]

    # **Value**
    #
    # This synthesized field doesn't have a description.
    value: NotRequired[Optional[int | None | std.IntNumberNullableWithAggregatesFilter[int]]]


# **Record relation filter**
#
# This synthesized interface doesn't have a description
class RecordRelationFilter(TypedDict):


    # **Is**
    #
    # This synthesized field doesn't have a description.
    is_: NotRequired[Optional[RecordWhereInput]]

    # **Is Not**
    #
    # This synthesized field doesn't have a description.
    isNot: NotRequired[Optional[RecordWhereInput]]


# **Record list relation filter**
#
# This synthesized interface doesn't have a description
class RecordListRelationFilter(TypedDict):


    # **Every**
    #
    # This synthesized field doesn't have a description.
    every: NotRequired[Optional[RecordWhereInput]]

    # **None**
    #
    # This synthesized field doesn't have a description.
    none: NotRequired[Optional[RecordWhereInput]]

    # **Some**
    #
    # This synthesized field doesn't have a description.
    some: NotRequired[Optional[RecordWhereInput]]


# **Record order by input**
#
# This synthesized interface doesn't have a description
class RecordOrderByInput(TypedDict):


    # **Created At**
    #
    # This synthesized field doesn't have a description.
    createdAt: NotRequired[Optional[std.Sort]]

    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[std.Sort]]

    # **Updated At**
    #
    # This synthesized field doesn't have a description.
    updatedAt: NotRequired[Optional[std.Sort]]

    # **Value**
    #
    # This synthesized field doesn't have a description.
    value: NotRequired[Optional[std.Sort]]


# **Record count aggregate input type**
#
# This synthesized interface doesn't have a description
class RecordCountAggregateInputType(TypedDict):


    # **All**
    #
    # This synthesized field doesn't have a description.
    _all: NotRequired[Optional[bool]]

    # **Created At**
    #
    # This synthesized field doesn't have a description.
    createdAt: NotRequired[Optional[bool]]

    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[bool]]

    # **Updated At**
    #
    # This synthesized field doesn't have a description.
    updatedAt: NotRequired[Optional[bool]]

    # **Value**
    #
    # This synthesized field doesn't have a description.
    value: NotRequired[Optional[bool]]


# **Record sum aggregate input type**
#
# This synthesized interface doesn't have a description
class RecordSumAggregateInputType(TypedDict):


    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[bool]]


# **Record avg aggregate input type**
#
# This synthesized interface doesn't have a description
class RecordAvgAggregateInputType(TypedDict):


    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[bool]]


# **Record min aggregate input type**
#
# This synthesized interface doesn't have a description
class RecordMinAggregateInputType(TypedDict):


    # **Created At**
    #
    # This synthesized field doesn't have a description.
    createdAt: NotRequired[Optional[bool]]

    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[bool]]

    # **Updated At**
    #
    # This synthesized field doesn't have a description.
    updatedAt: NotRequired[Optional[bool]]

    # **Value**
    #
    # This synthesized field doesn't have a description.
    value: NotRequired[Optional[bool]]


# **Record max aggregate input type**
#
# This synthesized interface doesn't have a description
class RecordMaxAggregateInputType(TypedDict):


    # **Created At**
    #
    # This synthesized field doesn't have a description.
    createdAt: NotRequired[Optional[bool]]

    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[bool]]

    # **Updated At**
    #
    # This synthesized field doesn't have a description.
    updatedAt: NotRequired[Optional[bool]]

    # **Value**
    #
    # This synthesized field doesn't have a description.
    value: NotRequired[Optional[bool]]


# **Record create input**
#
# This synthesized interface doesn't have a description
class RecordCreateInput(TypedDict):


    # **Value**
    #
    # This synthesized field doesn't have a description.
    value: NotRequired[Optional[int]]


# **Record update input**
#
# This synthesized interface doesn't have a description
class RecordUpdateInput(TypedDict):


    # **Value**
    #
    # This synthesized field doesn't have a description.
    value: NotRequired[Optional[int]]


# **Record create nested one input**
#
# This synthesized interface doesn't have a description
class RecordCreateNestedOneInput(TypedDict):


    # **Connect**
    #
    # This synthesized field doesn't have a description.
    connect: NotRequired[Optional[RecordWhereUniqueInput]]

    # **Connect Or Create**
    #
    # This synthesized field doesn't have a description.
    connectOrCreate: NotRequired[Optional[RecordConnectOrCreateInput]]

    # **Create**
    #
    # This synthesized field doesn't have a description.
    create: NotRequired[Optional[RecordCreateInput]]


# **Record create nested many input**
#
# This synthesized interface doesn't have a description
class RecordCreateNestedManyInput(TypedDict):


    # **Connect**
    #
    # This synthesized field doesn't have a description.
    connect: NotRequired[Optional[Enumerable[RecordWhereUniqueInput]]]

    # **Connect Or Create**
    #
    # This synthesized field doesn't have a description.
    connectOrCreate: NotRequired[Optional[Enumerable[RecordConnectOrCreateInput]]]

    # **Create**
    #
    # This synthesized field doesn't have a description.
    create: NotRequired[Optional[Enumerable[RecordCreateInput]]]


# **Record update nested one input**
#
# This synthesized interface doesn't have a description
class RecordUpdateNestedOneInput(TypedDict):


    # **Connect**
    #
    # This synthesized field doesn't have a description.
    connect: NotRequired[Optional[RecordWhereUniqueInput]]

    # **Connect Or Create**
    #
    # This synthesized field doesn't have a description.
    connectOrCreate: NotRequired[Optional[RecordConnectOrCreateInput]]

    # **Create**
    #
    # This synthesized field doesn't have a description.
    create: NotRequired[Optional[RecordCreateInput]]

    # **Delete**
    #
    # This synthesized field doesn't have a description.
    delete: NotRequired[Optional[bool]]

    # **Disconnect**
    #
    # This synthesized field doesn't have a description.
    disconnect: NotRequired[Optional[bool]]

    # **Set**
    #
    # This synthesized field doesn't have a description.
    set: NotRequired[Optional[RecordWhereUniqueInput]]

    # **Update**
    #
    # This synthesized field doesn't have a description.
    update: NotRequired[Optional[RecordUpdateInput]]

    # **Upsert**
    #
    # This synthesized field doesn't have a description.
    upsert: NotRequired[Optional[RecordUpsertWithWhereUniqueInput]]


# **Record update nested many input**
#
# This synthesized interface doesn't have a description
class RecordUpdateNestedManyInput(TypedDict):


    # **Connect**
    #
    # This synthesized field doesn't have a description.
    connect: NotRequired[Optional[Enumerable[RecordWhereUniqueInput]]]

    # **Connect Or Create**
    #
    # This synthesized field doesn't have a description.
    connectOrCreate: NotRequired[Optional[Enumerable[RecordConnectOrCreateInput]]]

    # **Create**
    #
    # This synthesized field doesn't have a description.
    create: NotRequired[Optional[Enumerable[RecordCreateInput]]]

    # **Delete**
    #
    # This synthesized field doesn't have a description.
    delete: NotRequired[Optional[Enumerable[RecordWhereUniqueInput]]]

    # **Delete Many**
    #
    # This synthesized field doesn't have a description.
    deleteMany: NotRequired[Optional[Enumerable[RecordWhereInput]]]

    # **Disconnect**
    #
    # This synthesized field doesn't have a description.
    disconnect: NotRequired[Optional[Enumerable[RecordWhereUniqueInput]]]

    # **Set**
    #
    # This synthesized field doesn't have a description.
    set: NotRequired[Optional[Enumerable[RecordWhereUniqueInput]]]

    # **Update**
    #
    # This synthesized field doesn't have a description.
    update: NotRequired[Optional[Enumerable[RecordUpdateWithWhereUniqueInput]]]

    # **Update Many**
    #
    # This synthesized field doesn't have a description.
    updateMany: NotRequired[Optional[Enumerable[RecordUpdateManyWithWhereInput]]]

    # **Upsert**
    #
    # This synthesized field doesn't have a description.
    upsert: NotRequired[Optional[Enumerable[RecordUpsertWithWhereUniqueInput]]]


# **Record connect or create input**
#
# This synthesized interface doesn't have a description
class RecordConnectOrCreateInput(TypedDict):


    # **Create**
    #
    # This synthesized field doesn't have a description.
    create: RecordCreateInput

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: RecordWhereUniqueInput


# **Record update with where unique input**
#
# This synthesized interface doesn't have a description
class RecordUpdateWithWhereUniqueInput(TypedDict):


    # **Update**
    #
    # This synthesized field doesn't have a description.
    update: RecordUpdateInput

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: RecordWhereUniqueInput


# **Record upsert with where unique input**
#
# This synthesized interface doesn't have a description
class RecordUpsertWithWhereUniqueInput(TypedDict):


    # **Create**
    #
    # This synthesized field doesn't have a description.
    create: RecordCreateInput

    # **Update**
    #
    # This synthesized field doesn't have a description.
    update: RecordUpdateInput

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: RecordWhereUniqueInput


# **Record update many with where input**
#
# This synthesized interface doesn't have a description
class RecordUpdateManyWithWhereInput(TypedDict):


    # **Update**
    #
    # This synthesized field doesn't have a description.
    update: RecordUpdateInput

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: RecordWhereInput


# **Record result**
#
# This synthesized interface doesn't have a description
class RecordResult(TypedDict):


    # **Created At**
    #
    # This synthesized field doesn't have a description.
    createdAt: datetime

    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: int

    # **Updated At**
    #
    # This synthesized field doesn't have a description.
    updatedAt: datetime

    # **Value**
    #
    # This synthesized field doesn't have a description.
    value: NotRequired[Optional[int]]


# **Record count aggregate result**
#
# This synthesized interface doesn't have a description
class RecordCountAggregateResult(TypedDict):


    # **All**
    #
    # This synthesized field doesn't have a description.
    _all: NotRequired[Optional[int]]

    # **Created At**
    #
    # This synthesized field doesn't have a description.
    createdAt: NotRequired[Optional[int]]

    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[int]]

    # **Updated At**
    #
    # This synthesized field doesn't have a description.
    updatedAt: NotRequired[Optional[int]]

    # **Value**
    #
    # This synthesized field doesn't have a description.
    value: NotRequired[Optional[int]]


# **Record sum aggregate result**
#
# This synthesized interface doesn't have a description
class RecordSumAggregateResult(TypedDict):


    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[int]]


# **Record avg aggregate result**
#
# This synthesized interface doesn't have a description
class RecordAvgAggregateResult(TypedDict):


    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[float]]


# **Record min aggregate result**
#
# This synthesized interface doesn't have a description
class RecordMinAggregateResult(TypedDict):


    # **Created At**
    #
    # This synthesized field doesn't have a description.
    createdAt: NotRequired[Optional[datetime]]

    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[int]]

    # **Updated At**
    #
    # This synthesized field doesn't have a description.
    updatedAt: NotRequired[Optional[datetime]]

    # **Value**
    #
    # This synthesized field doesn't have a description.
    value: NotRequired[Optional[int]]


# **Record max aggregate result**
#
# This synthesized interface doesn't have a description
class RecordMaxAggregateResult(TypedDict):


    # **Created At**
    #
    # This synthesized field doesn't have a description.
    createdAt: NotRequired[Optional[datetime]]

    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[int]]

    # **Updated At**
    #
    # This synthesized field doesn't have a description.
    updatedAt: NotRequired[Optional[datetime]]

    # **Value**
    #
    # This synthesized field doesn't have a description.
    value: NotRequired[Optional[int]]


# **Record aggregate result**
#
# This synthesized interface doesn't have a description
class RecordAggregateResult(TypedDict):


    # **Avg**
    #
    # This synthesized field doesn't have a description.
    _avg: NotRequired[Optional[RecordAvgAggregateResult]]

    # **Count**
    #
    # This synthesized field doesn't have a description.
    _count: NotRequired[Optional[RecordCountAggregateResult]]

    # **Max**
    #
    # This synthesized field doesn't have a description.
    _max: NotRequired[Optional[RecordMaxAggregateResult]]

    # **Min**
    #
    # This synthesized field doesn't have a description.
    _min: NotRequired[Optional[RecordMinAggregateResult]]

    # **Sum**
    #
    # This synthesized field doesn't have a description.
    _sum: NotRequired[Optional[RecordSumAggregateResult]]


# **Record group by result**
#
# This synthesized interface doesn't have a description
class RecordGroupByResult(TypedDict):


    # **Avg**
    #
    # This synthesized field doesn't have a description.
    _avg: NotRequired[Optional[RecordAvgAggregateResult]]

    # **Count**
    #
    # This synthesized field doesn't have a description.
    _count: NotRequired[Optional[RecordCountAggregateResult]]

    # **Max**
    #
    # This synthesized field doesn't have a description.
    _max: NotRequired[Optional[RecordMaxAggregateResult]]

    # **Min**
    #
    # This synthesized field doesn't have a description.
    _min: NotRequired[Optional[RecordMinAggregateResult]]

    # **Sum**
    #
    # This synthesized field doesn't have a description.
    _sum: NotRequired[Optional[RecordSumAggregateResult]]

    # **Created At**
    #
    # This synthesized field doesn't have a description.
    createdAt: NotRequired[Optional[datetime]]

    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[int]]

    # **Updated At**
    #
    # This synthesized field doesn't have a description.
    updatedAt: NotRequired[Optional[datetime]]

    # **Value**
    #
    # This synthesized field doesn't have a description.
    value: NotRequired[Optional[int]]


# **Record args**
#
# This synthesized interface doesn't have a description
class RecordArgs(TypedDict):


    # **Include**
    #
    # This synthesized field doesn't have a description.
    include: NotRequired[Optional[RecordInclude]]

    # **Select**
    #
    # This synthesized field doesn't have a description.
    select: NotRequired[Optional[RecordSelect]]


# **Record find many args**
#
# This synthesized interface doesn't have a description
class RecordFindManyArgs(TypedDict):


    # **Cursor**
    #
    # This synthesized field doesn't have a description.
    cursor: NotRequired[Optional[RecordWhereUniqueInput]]

    # **Distinct**
    #
    # This synthesized field doesn't have a description.
    distinct: NotRequired[Optional[RecordSerializableScalarFields]]

    # **Include**
    #
    # This synthesized field doesn't have a description.
    include: NotRequired[Optional[RecordInclude]]

    # **Order By**
    #
    # This synthesized field doesn't have a description.
    orderBy: NotRequired[Optional[Enumerable[RecordOrderByInput]]]

    # **Page Number**
    #
    # This synthesized field doesn't have a description.
    pageNumber: NotRequired[Optional[int]]

    # **Page Size**
    #
    # This synthesized field doesn't have a description.
    pageSize: NotRequired[Optional[int]]

    # **Select**
    #
    # This synthesized field doesn't have a description.
    select: NotRequired[Optional[RecordSelect]]

    # **Skip**
    #
    # This synthesized field doesn't have a description.
    skip: NotRequired[Optional[int]]

    # **Take**
    #
    # This synthesized field doesn't have a description.
    take: NotRequired[Optional[int]]

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: NotRequired[Optional[RecordWhereInput]]


# **Record find first args**
#
# This synthesized interface doesn't have a description
class RecordFindFirstArgs(TypedDict):


    # **Cursor**
    #
    # This synthesized field doesn't have a description.
    cursor: NotRequired[Optional[RecordWhereUniqueInput]]

    # **Distinct**
    #
    # This synthesized field doesn't have a description.
    distinct: NotRequired[Optional[RecordSerializableScalarFields]]

    # **Include**
    #
    # This synthesized field doesn't have a description.
    include: NotRequired[Optional[RecordInclude]]

    # **Order By**
    #
    # This synthesized field doesn't have a description.
    orderBy: NotRequired[Optional[Enumerable[RecordOrderByInput]]]

    # **Page Number**
    #
    # This synthesized field doesn't have a description.
    pageNumber: NotRequired[Optional[int]]

    # **Page Size**
    #
    # This synthesized field doesn't have a description.
    pageSize: NotRequired[Optional[int]]

    # **Select**
    #
    # This synthesized field doesn't have a description.
    select: NotRequired[Optional[RecordSelect]]

    # **Skip**
    #
    # This synthesized field doesn't have a description.
    skip: NotRequired[Optional[int]]

    # **Take**
    #
    # This synthesized field doesn't have a description.
    take: NotRequired[Optional[int]]

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: NotRequired[Optional[RecordWhereInput]]


# **Record find unique args**
#
# This synthesized interface doesn't have a description
class RecordFindUniqueArgs(TypedDict):


    # **Include**
    #
    # This synthesized field doesn't have a description.
    include: NotRequired[Optional[RecordInclude]]

    # **Select**
    #
    # This synthesized field doesn't have a description.
    select: NotRequired[Optional[RecordSelect]]

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: RecordWhereUniqueInput


# **Record create args**
#
# This synthesized interface doesn't have a description
class RecordCreateArgs(TypedDict):


    # **Create**
    #
    # This synthesized field doesn't have a description.
    create: RecordCreateInput

    # **Include**
    #
    # This synthesized field doesn't have a description.
    include: NotRequired[Optional[RecordInclude]]

    # **Select**
    #
    # This synthesized field doesn't have a description.
    select: NotRequired[Optional[RecordSelect]]


# **Record update args**
#
# This synthesized interface doesn't have a description
class RecordUpdateArgs(TypedDict):


    # **Include**
    #
    # This synthesized field doesn't have a description.
    include: NotRequired[Optional[RecordInclude]]

    # **Select**
    #
    # This synthesized field doesn't have a description.
    select: NotRequired[Optional[RecordSelect]]

    # **Update**
    #
    # This synthesized field doesn't have a description.
    update: RecordUpdateInput

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: RecordWhereUniqueInput


# **Record upsert args**
#
# This synthesized interface doesn't have a description
class RecordUpsertArgs(TypedDict):


    # **Create**
    #
    # This synthesized field doesn't have a description.
    create: RecordCreateInput

    # **Include**
    #
    # This synthesized field doesn't have a description.
    include: NotRequired[Optional[RecordInclude]]

    # **Select**
    #
    # This synthesized field doesn't have a description.
    select: NotRequired[Optional[RecordSelect]]

    # **Update**
    #
    # This synthesized field doesn't have a description.
    update: RecordUpdateInput

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: RecordWhereUniqueInput


# **Record copy args**
#
# This synthesized interface doesn't have a description
class RecordCopyArgs(TypedDict):


    # **Copy**
    #
    # This synthesized field doesn't have a description.
    copy: RecordUpdateInput

    # **Include**
    #
    # This synthesized field doesn't have a description.
    include: NotRequired[Optional[RecordInclude]]

    # **Select**
    #
    # This synthesized field doesn't have a description.
    select: NotRequired[Optional[RecordSelect]]

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: RecordWhereUniqueInput


# **Record delete args**
#
# This synthesized interface doesn't have a description
class RecordDeleteArgs(TypedDict):


    # **Include**
    #
    # This synthesized field doesn't have a description.
    include: NotRequired[Optional[RecordInclude]]

    # **Select**
    #
    # This synthesized field doesn't have a description.
    select: NotRequired[Optional[RecordSelect]]

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: RecordWhereUniqueInput


# **Record create many args**
#
# This synthesized interface doesn't have a description
class RecordCreateManyArgs(TypedDict):


    # **Create**
    #
    # This synthesized field doesn't have a description.
    create: Enumerable[RecordCreateInput]

    # **Include**
    #
    # This synthesized field doesn't have a description.
    include: NotRequired[Optional[RecordInclude]]

    # **Select**
    #
    # This synthesized field doesn't have a description.
    select: NotRequired[Optional[RecordSelect]]


# **Record update many args**
#
# This synthesized interface doesn't have a description
class RecordUpdateManyArgs(TypedDict):


    # **Include**
    #
    # This synthesized field doesn't have a description.
    include: NotRequired[Optional[RecordInclude]]

    # **Select**
    #
    # This synthesized field doesn't have a description.
    select: NotRequired[Optional[RecordSelect]]

    # **Update**
    #
    # This synthesized field doesn't have a description.
    update: RecordUpdateInput

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: RecordWhereInput


# **Record delete many args**
#
# This synthesized interface doesn't have a description
class RecordDeleteManyArgs(TypedDict):


    # **Include**
    #
    # This synthesized field doesn't have a description.
    include: NotRequired[Optional[RecordInclude]]

    # **Select**
    #
    # This synthesized field doesn't have a description.
    select: NotRequired[Optional[RecordSelect]]

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: RecordWhereInput


# **Record copy many args**
#
# This synthesized interface doesn't have a description
class RecordCopyManyArgs(TypedDict):


    # **Copy**
    #
    # This synthesized field doesn't have a description.
    copy: RecordUpdateInput

    # **Include**
    #
    # This synthesized field doesn't have a description.
    include: NotRequired[Optional[RecordInclude]]

    # **Select**
    #
    # This synthesized field doesn't have a description.
    select: NotRequired[Optional[RecordSelect]]

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: RecordWhereInput


# **Record count args**
#
# This synthesized interface doesn't have a description
class RecordCountArgs(TypedDict):


    # **Cursor**
    #
    # This synthesized field doesn't have a description.
    cursor: NotRequired[Optional[RecordWhereUniqueInput]]

    # **Distinct**
    #
    # This synthesized field doesn't have a description.
    distinct: NotRequired[Optional[RecordSerializableScalarFields]]

    # **Order By**
    #
    # This synthesized field doesn't have a description.
    orderBy: NotRequired[Optional[Enumerable[RecordOrderByInput]]]

    # **Page Number**
    #
    # This synthesized field doesn't have a description.
    pageNumber: NotRequired[Optional[int]]

    # **Page Size**
    #
    # This synthesized field doesn't have a description.
    pageSize: NotRequired[Optional[int]]

    # **Select**
    #
    # This synthesized field doesn't have a description.
    select: NotRequired[Optional[RecordCountAggregateInputType]]

    # **Skip**
    #
    # This synthesized field doesn't have a description.
    skip: NotRequired[Optional[int]]

    # **Take**
    #
    # This synthesized field doesn't have a description.
    take: NotRequired[Optional[int]]

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: NotRequired[Optional[RecordWhereInput]]


# **Record aggregate args**
#
# This synthesized interface doesn't have a description
class RecordAggregateArgs(TypedDict):


    # **Avg**
    #
    # This synthesized field doesn't have a description.
    _avg: NotRequired[Optional[RecordAvgAggregateInputType]]

    # **Count**
    #
    # This synthesized field doesn't have a description.
    _count: NotRequired[Optional[RecordCountAggregateInputType]]

    # **Max**
    #
    # This synthesized field doesn't have a description.
    _max: NotRequired[Optional[RecordMaxAggregateInputType]]

    # **Min**
    #
    # This synthesized field doesn't have a description.
    _min: NotRequired[Optional[RecordMinAggregateInputType]]

    # **Sum**
    #
    # This synthesized field doesn't have a description.
    _sum: NotRequired[Optional[RecordSumAggregateInputType]]

    # **Cursor**
    #
    # This synthesized field doesn't have a description.
    cursor: NotRequired[Optional[RecordWhereUniqueInput]]

    # **Distinct**
    #
    # This synthesized field doesn't have a description.
    distinct: NotRequired[Optional[RecordSerializableScalarFields]]

    # **Order By**
    #
    # This synthesized field doesn't have a description.
    orderBy: NotRequired[Optional[Enumerable[RecordOrderByInput]]]

    # **Page Number**
    #
    # This synthesized field doesn't have a description.
    pageNumber: NotRequired[Optional[int]]

    # **Page Size**
    #
    # This synthesized field doesn't have a description.
    pageSize: NotRequired[Optional[int]]

    # **Skip**
    #
    # This synthesized field doesn't have a description.
    skip: NotRequired[Optional[int]]

    # **Take**
    #
    # This synthesized field doesn't have a description.
    take: NotRequired[Optional[int]]

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: NotRequired[Optional[RecordWhereInput]]


# **Record group by args**
#
# This synthesized interface doesn't have a description
class RecordGroupByArgs(TypedDict):


    # **Avg**
    #
    # This synthesized field doesn't have a description.
    _avg: NotRequired[Optional[RecordAvgAggregateInputType]]

    # **Count**
    #
    # This synthesized field doesn't have a description.
    _count: NotRequired[Optional[RecordCountAggregateInputType]]

    # **Max**
    #
    # This synthesized field doesn't have a description.
    _max: NotRequired[Optional[RecordMaxAggregateInputType]]

    # **Min**
    #
    # This synthesized field doesn't have a description.
    _min: NotRequired[Optional[RecordMinAggregateInputType]]

    # **Sum**
    #
    # This synthesized field doesn't have a description.
    _sum: NotRequired[Optional[RecordSumAggregateInputType]]

    # **By**
    #
    # This synthesized field doesn't have a description.
    by: Enumerable[RecordSerializableScalarFields]

    # **Cursor**
    #
    # This synthesized field doesn't have a description.
    cursor: NotRequired[Optional[RecordWhereUniqueInput]]

    # **Distinct**
    #
    # This synthesized field doesn't have a description.
    distinct: NotRequired[Optional[RecordSerializableScalarFields]]

    # **Having**
    #
    # This synthesized field doesn't have a description.
    having: NotRequired[Optional[RecordScalarWhereWithAggregatesInput]]

    # **Order By**
    #
    # This synthesized field doesn't have a description.
    orderBy: NotRequired[Optional[Enumerable[RecordOrderByInput]]]

    # **Page Number**
    #
    # This synthesized field doesn't have a description.
    pageNumber: NotRequired[Optional[int]]

    # **Page Size**
    #
    # This synthesized field doesn't have a description.
    pageSize: NotRequired[Optional[int]]

    # **Skip**
    #
    # This synthesized field doesn't have a description.
    skip: NotRequired[Optional[int]]

    # **Take**
    #
    # This synthesized field doesn't have a description.
    take: NotRequired[Optional[int]]

    # **Where**
    #
    # This synthesized field doesn't have a description.
    where: NotRequired[Optional[RecordWhereInput]]


# **Record scalar update input**
#
# This synthesized interface doesn't have a description
class RecordScalarUpdateInput(TypedDict):


    # **Created At**
    #
    # This synthesized field doesn't have a description.
    createdAt: NotRequired[Optional[datetime]]

    # **Id**
    #
    # This synthesized field doesn't have a description.
    id: NotRequired[Optional[int]]

    # **Updated At**
    #
    # This synthesized field doesn't have a description.
    updatedAt: NotRequired[Optional[datetime]]

    # **Value**
    #
    # This synthesized field doesn't have a description.
    value: NotRequired[Optional[int]]


# **Record sign in checker ids**
#
# This synthesized interface doesn't have a description
class RecordSignInCheckerIds(TypedDict):

    pass



# **Record sign in checker companions**
#
# This synthesized interface doesn't have a description
class RecordSignInCheckerCompanions(TypedDict):

    pass



# **Record sign in input**
#
# This synthesized interface doesn't have a description
class RecordSignInInput(TypedDict):


    # **Credentials**
    #
    # This synthesized field doesn't have a description.
    credentials: RecordSignInArgs

    # **Include**
    #
    # This synthesized field doesn't have a description.
    include: NotRequired[Optional[RecordInclude]]

    # **Select**
    #
    # This synthesized field doesn't have a description.
    select: NotRequired[Optional[RecordSelect]]


# **Record sign in args**
#
# This synthesized interface doesn't have a description
class RecordSignInArgs(TypedDict):

    pass





class RecordModel:
    async def find_many_objects(self, query: RecordFindManyArgs, /) -> list[Record]:
        return cast(Any, None)
    async def find_unique_object(self, query: RecordFindUniqueArgs, /) -> Optional[Record]:
        return cast(Any, None)
    async def find_first_object(self, query: RecordFindFirstArgs, /) -> Optional[Record]:
        return cast(Any, None)
    async def create_object(self, input: RecordCreateInput, /) -> Record:
        return cast(Any, None)
    async def count_objects(self, query: RecordCountArgs, /) -> int:
        return cast(Any, None)
    async def count_fields(self, query: RecordCountArgs, /) -> RecordCountAggregateResult:
        return cast(Any, None)
    async def aggregate(self, query: RecordAggregateArgs, /) -> RecordAggregateResult:
        return cast(Any, None)
    async def group_by(self, query: RecordGroupByArgs, /) -> list[RecordAggregateResult]:
        return cast(Any, None)
    
    async def sql(self, sql: str) -> list[Any]:
        return cast(Any, None)
    
class Record:
    def is_new(self) -> bool:
        return cast(Any, None)
    def is_modified(self) -> bool:
        return cast(Any, None)
    async def set(self, input: RecordUpdateInput, /) -> None:
        return cast(Any, None)
    async def update(self, input: RecordScalarUpdateInput, /) -> None:
        return cast(Any, None)
    async def save(self) -> None:
        return cast(Any, None)
    async def delete(self) -> None:
        return cast(Any, None)
    async def to_teon(self) -> RecordResult:
        return cast(Any, None)
    id: int
    value: Optional[int]
    created_at: datetime
    updated_at: datetime



class EchoPathArguments(TypedDict):

    data: str


class StaticPathArguments(TypedDict):

    path: str


class Teo:

    
    async def transaction[T](self, teo: Teo, /) -> T:
        return cast(Any, None)
    
    record: RecordModel