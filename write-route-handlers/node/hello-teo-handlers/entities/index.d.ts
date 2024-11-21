import Decimal from "decimal.js"
import { DateOnly, ObjectId, File } from "@teocloud/teo"

export type ExistKeys<T> = {
    [key in keyof T]: T[key] extends false | undefined | null ? never : key
}[keyof T]

type HasSelect = {
    select: any
}

type HasInclude = {
    include: any
}

export type CheckSelectInclude<T, S, U> = T extends HasSelect
    ? U
    : T extends HasInclude
    ? U
    : S

export type SelectSubset<T, U> = U extends HasSelect
    ? {
        [K in ExistKeys<U['select']>]: K extends keyof T ? T[K] : never
    }
    : T

export type Enumerable<T> = T | Array<T>

export type Subset<T, U> = {
    [key in keyof T]: key extends keyof U ? T[key] : never
}

export type GetScalarType<T, O> = O extends object ? {
    [P in keyof T]: P extends keyof O
      ? O[P]
      : never
} : never

type Teo__Pick<T, K extends keyof T> = {
    [P in K]: T[P];
}

type PickEnumerable<T, K extends Enumerable<keyof T> | keyof T> = Teo__Pick<T, MaybeTupleToUnion<K>>

export type Boolean = True | False

export type True = 1

export type False = 0

export type Not<B extends Boolean> = {
  0: 1
  1: 0
}[B]

type NoExpand<T> = T extends unknown ? T : never;

type Key = string | number | symbol;
type AtBasic<O extends object, K extends Key> = K extends keyof O ? O[K] : never;
type AtStrict<O extends object, K extends Key> = O[K & keyof O];
type AtLoose<O extends object, K extends Key> = O extends unknown ? AtStrict<O, K> : never;
export type At<O extends object, K extends Key, strict extends Boolean = 1> = {
    1: AtStrict<O, K>;
    0: AtLoose<O, K>;
}[strict];

export type IntersectOf<U extends Union> = (
  U extends unknown ? (k: U) => void : never
) extends (k: infer I) => void
  ? I
  : never

export type Overwrite<O extends object, O1 extends object> = {
    [K in keyof O]: K extends keyof O1 ? O1[K] : O[K];
} & {};

type _Merge<U extends object> = IntersectOf<Overwrite<U, {
    [K in keyof U]-?: At<U, K>;
}>>;

export type ComputeRaw<A extends any> = A extends Function ? A : {
  [K in keyof A]: A[K];
} & {};

export type OptionalFlat<O> = {
  [K in keyof O]?: O[K];
} & {};

type _Record<K extends keyof any, T> = {
  [P in K]: T;
};

type AtLeast<O extends object, K extends string> = NoExpand<
  O extends unknown
  ? | (K extends keyof O ? { [P in K]: O[P] } & O : O)
    | {[P in keyof O as P extends K ? K : never]-?: O[P]} & O
  : never>;

type _Strict<U, _U = U> = U extends unknown ? U & OptionalFlat<_Record<Exclude<Keys<_U>, keyof U>, never>> : never;

export type Strict<U extends object> = ComputeRaw<_Strict<U>>;

export type Merge<U extends object> = ComputeRaw<_Merge<Strict<U>>>;

type ExcludeUnderscoreKeys<T extends string> = T extends `_${string}` ? never : T

export type Union = any

export type Extends<A1 extends any, A2 extends any> = [A1] extends [never]
  ? 0 // anything `never` is false
  : A1 extends A2
  ? 1
  : 0

export type Has<U extends Union, U1 extends Union> = Not<
  Extends<Exclude<U1, U>, U1>
>

export type Or<B1 extends Boolean, B2 extends Boolean> = {
  0: {
    0: 0
    1: 1
  }
  1: {
    0: 1
    1: 1
  }
}[B1][B2]

export type Keys<U extends Union> = U extends unknown ? keyof U : never

type Cast<A, B> = A extends B ? A : B;

type IsObject<T extends any> = T extends Array<any>
? False
: T extends Date
? False
: T extends Uint8Array
? False
: T extends object
? True
: False

type FieldPaths<
  T,
  U = Omit<T, '_avg' | '_sum' | '_count' | '_min' | '_max'>
> = IsObject<T> extends True ? U : T

type GetHavingFields<T> = {
  [K in keyof T]: Or<
    Or<Extends<'OR', K>, Extends<'AND', K>>,
    Extends<'NOT', K>
  > extends True
    ? // infer is only needed to not hit TS limit
      // based on the brilliant idea of Pierre-Antoine Mills
      // https://github.com/microsoft/TypeScript/issues/30188#issuecomment-478938437
      T[K] extends infer TK
      ? GetHavingFields<UnEnumerate<TK> extends object ? Merge<UnEnumerate<TK>> : never>
      : never
    : {} extends FieldPaths<T[K]>
    ? never
    : K
}[keyof T]

export type UnEnumerate<T extends unknown> = T extends Array<infer U> ? U : T

export type SubsetIntersection<T, U, K> = {
  [key in keyof T]: key extends keyof U ? T[key] : never
} & K

type _TupleToUnion<T> = T extends (infer E)[] ? E : never
type TupleToUnion<K extends readonly any[]> = _TupleToUnion<K>
type MaybeTupleToUnion<T> = T extends any[] ? TupleToUnion<T> : T

/**
 * **Record scalar fields**
 *
 * This synthesized enum doesn't have a description.
 */
export type RecordScalarFields = "createdAt" | "id" | "updatedAt" | "value"

/**
 * **Record serializable scalar fields**
 *
 * This synthesized enum doesn't have a description.
 */
export type RecordSerializableScalarFields = "createdAt" | "id" | "updatedAt" | "value"

/**
 * **Record relations**
 *
 * This synthesized enum doesn't have a description.
 */
export type RecordRelations = undefined

/**
 * **Record direct relations**
 *
 * This synthesized enum doesn't have a description.
 */
export type RecordDirectRelations = undefined

/**
 * **Record indirect relations**
 *
 * This synthesized enum doesn't have a description.
 */
export type RecordIndirectRelations = undefined

/// ## Record scalar fields
///
/// This synthesized enum doesn't have a description.
export const enum RecordScalarFieldsEnumType {

    /// ### Created at
    ///
    /// This synthesized enum member doesn't have a description.
    createdAt = "createdAt",

    /// ### Id
    ///
    /// This synthesized enum member doesn't have a description.
    id = "id",

    /// ### Updated at
    ///
    /// This synthesized enum member doesn't have a description.
    updatedAt = "updatedAt",

    /// ### Value
    ///
    /// This synthesized enum member doesn't have a description.
    value = "value",
}

/// ## Record serializable scalar fields
///
/// This synthesized enum doesn't have a description.
export const enum RecordSerializableScalarFieldsEnumType {

    /// ### Created at
    ///
    /// This synthesized enum member doesn't have a description.
    createdAt = "createdAt",

    /// ### Id
    ///
    /// This synthesized enum member doesn't have a description.
    id = "id",

    /// ### Updated at
    ///
    /// This synthesized enum member doesn't have a description.
    updatedAt = "updatedAt",

    /// ### Value
    ///
    /// This synthesized enum member doesn't have a description.
    value = "value",
}

/// ## Record relations
///
/// This synthesized enum doesn't have a description.
export const enum RecordRelationsEnumType {
}

/// ## Record direct relations
///
/// This synthesized enum doesn't have a description.
export const enum RecordDirectRelationsEnumType {
}

/// ## Record indirect relations
///
/// This synthesized enum doesn't have a description.
export const enum RecordIndirectRelationsEnumType {
}


/**
 * **Alter created at input**
 *
 * This interface doesn't have a description.
 */
export type AlterCreatedAtInput = {
    
    /**
     * **Id**
     *
     * This interface field doesn't have a description.
     */
     id: number
    
    /**
     * **Created at**
     *
     * This interface field doesn't have a description.
     */
     createdAt: Date
    
}


/**
 * **Upload input**
 *
 * This interface doesn't have a description.
 */
export type UploadInput = {
    
    /**
     * **File**
     *
     * This interface field doesn't have a description.
     */
     file: File
    
}


/**
 * **Upload output**
 *
 * This interface doesn't have a description.
 */
export type UploadOutput = {
    
    /**
     * **Path**
     *
     * This interface field doesn't have a description.
     */
     path: string
    
}


/**
 * **Record select**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordSelect = {
    
    /**
     * **Created At**
     *
     * This synthesized field doesn't have a description.
     */
     createdAt?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Updated At**
     *
     * This synthesized field doesn't have a description.
     */
     updatedAt?: boolean
    
    /**
     * **Value**
     *
     * This synthesized field doesn't have a description.
     */
     value?: boolean
    
}


/**
 * **Record include**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordInclude = {
    
}


/**
 * **Record where input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordWhereInput = {
    
    /**
     * **And**
     *
     * This synthesized field doesn't have a description.
     */
     AND?: RecordWhereInput[]
    
    /**
     * **Not**
     *
     * This synthesized field doesn't have a description.
     */
     NOT?: RecordWhereInput
    
    /**
     * **Or**
     *
     * This synthesized field doesn't have a description.
     */
     OR?: RecordWhereInput[]
    
    /**
     * **Created At**
     *
     * This synthesized field doesn't have a description.
     */
     createdAt?: Date | std.Filter<Date>
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number | std.Filter<number>
    
    /**
     * **Updated At**
     *
     * This synthesized field doesn't have a description.
     */
     updatedAt?: Date | std.Filter<Date>
    
    /**
     * **Value**
     *
     * This synthesized field doesn't have a description.
     */
     value?: number | null | std.NullableFilter<number>
    
}


/**
 * **Record where unique input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordWhereUniqueInput = {
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id: number
    
}


/**
 * **Record scalar where with aggregates input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordScalarWhereWithAggregatesInput = {
    
    /**
     * **And**
     *
     * This synthesized field doesn't have a description.
     */
     AND?: RecordWhereInput[]
    
    /**
     * **Not**
     *
     * This synthesized field doesn't have a description.
     */
     NOT?: RecordWhereInput
    
    /**
     * **Or**
     *
     * This synthesized field doesn't have a description.
     */
     OR?: RecordWhereInput[]
    
    /**
     * **Created At**
     *
     * This synthesized field doesn't have a description.
     */
     createdAt?: Date | std.AggregatesFilter<Date>
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number | std.IntNumberWithAggregatesFilter<number>
    
    /**
     * **Updated At**
     *
     * This synthesized field doesn't have a description.
     */
     updatedAt?: Date | std.AggregatesFilter<Date>
    
    /**
     * **Value**
     *
     * This synthesized field doesn't have a description.
     */
     value?: number | null | std.IntNumberNullableWithAggregatesFilter<number>
    
}


/**
 * **Record relation filter**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordRelationFilter = {
    
    /**
     * **Is**
     *
     * This synthesized field doesn't have a description.
     */
     is?: RecordWhereInput
    
    /**
     * **Is Not**
     *
     * This synthesized field doesn't have a description.
     */
     isNot?: RecordWhereInput
    
}


/**
 * **Record list relation filter**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordListRelationFilter = {
    
    /**
     * **Every**
     *
     * This synthesized field doesn't have a description.
     */
     every?: RecordWhereInput
    
    /**
     * **None**
     *
     * This synthesized field doesn't have a description.
     */
     none?: RecordWhereInput
    
    /**
     * **Some**
     *
     * This synthesized field doesn't have a description.
     */
     some?: RecordWhereInput
    
}


/**
 * **Record order by input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordOrderByInput = {
    
    /**
     * **Created At**
     *
     * This synthesized field doesn't have a description.
     */
     createdAt?: std.Sort
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: std.Sort
    
    /**
     * **Updated At**
     *
     * This synthesized field doesn't have a description.
     */
     updatedAt?: std.Sort
    
    /**
     * **Value**
     *
     * This synthesized field doesn't have a description.
     */
     value?: std.Sort
    
}


/**
 * **Record count aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordCountAggregateInputType = {
    
    /**
     * **All**
     *
     * This synthesized field doesn't have a description.
     */
     _all?: boolean
    
    /**
     * **Created At**
     *
     * This synthesized field doesn't have a description.
     */
     createdAt?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Updated At**
     *
     * This synthesized field doesn't have a description.
     */
     updatedAt?: boolean
    
    /**
     * **Value**
     *
     * This synthesized field doesn't have a description.
     */
     value?: boolean
    
}


/**
 * **Record sum aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordSumAggregateInputType = {
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
}


/**
 * **Record avg aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordAvgAggregateInputType = {
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
}


/**
 * **Record min aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordMinAggregateInputType = {
    
    /**
     * **Created At**
     *
     * This synthesized field doesn't have a description.
     */
     createdAt?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Updated At**
     *
     * This synthesized field doesn't have a description.
     */
     updatedAt?: boolean
    
    /**
     * **Value**
     *
     * This synthesized field doesn't have a description.
     */
     value?: boolean
    
}


/**
 * **Record max aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordMaxAggregateInputType = {
    
    /**
     * **Created At**
     *
     * This synthesized field doesn't have a description.
     */
     createdAt?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Updated At**
     *
     * This synthesized field doesn't have a description.
     */
     updatedAt?: boolean
    
    /**
     * **Value**
     *
     * This synthesized field doesn't have a description.
     */
     value?: boolean
    
}


/**
 * **Record create input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordCreateInput = {
    
    /**
     * **Value**
     *
     * This synthesized field doesn't have a description.
     */
     value?: number
    
}


/**
 * **Record update input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordUpdateInput = {
    
    /**
     * **Value**
     *
     * This synthesized field doesn't have a description.
     */
     value?: number
    
}


/**
 * **Record create nested one input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordCreateNestedOneInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: RecordWhereUniqueInput
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: RecordConnectOrCreateInput
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: RecordCreateInput
    
}


/**
 * **Record create nested many input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordCreateNestedManyInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: Enumerable<RecordWhereUniqueInput>
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: Enumerable<RecordConnectOrCreateInput>
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: Enumerable<RecordCreateInput>
    
}


/**
 * **Record update nested one input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordUpdateNestedOneInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: RecordWhereUniqueInput
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: RecordConnectOrCreateInput
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: RecordCreateInput
    
    /**
     * **Delete**
     *
     * This synthesized field doesn't have a description.
     */
     delete?: boolean
    
    /**
     * **Disconnect**
     *
     * This synthesized field doesn't have a description.
     */
     disconnect?: boolean
    
    /**
     * **Set**
     *
     * This synthesized field doesn't have a description.
     */
     set?: RecordWhereUniqueInput
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update?: RecordUpdateInput
    
    /**
     * **Upsert**
     *
     * This synthesized field doesn't have a description.
     */
     upsert?: RecordUpsertWithWhereUniqueInput
    
}


/**
 * **Record update nested many input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordUpdateNestedManyInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: Enumerable<RecordWhereUniqueInput>
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: Enumerable<RecordConnectOrCreateInput>
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: Enumerable<RecordCreateInput>
    
    /**
     * **Delete**
     *
     * This synthesized field doesn't have a description.
     */
     delete?: Enumerable<RecordWhereUniqueInput>
    
    /**
     * **Delete Many**
     *
     * This synthesized field doesn't have a description.
     */
     deleteMany?: Enumerable<RecordWhereInput>
    
    /**
     * **Disconnect**
     *
     * This synthesized field doesn't have a description.
     */
     disconnect?: Enumerable<RecordWhereUniqueInput>
    
    /**
     * **Set**
     *
     * This synthesized field doesn't have a description.
     */
     set?: Enumerable<RecordWhereUniqueInput>
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update?: Enumerable<RecordUpdateWithWhereUniqueInput>
    
    /**
     * **Update Many**
     *
     * This synthesized field doesn't have a description.
     */
     updateMany?: Enumerable<RecordUpdateManyWithWhereInput>
    
    /**
     * **Upsert**
     *
     * This synthesized field doesn't have a description.
     */
     upsert?: Enumerable<RecordUpsertWithWhereUniqueInput>
    
}


/**
 * **Record connect or create input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordConnectOrCreateInput = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: RecordCreateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: RecordWhereUniqueInput
    
}


/**
 * **Record update with where unique input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordUpdateWithWhereUniqueInput = {
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: RecordUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: RecordWhereUniqueInput
    
}


/**
 * **Record upsert with where unique input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordUpsertWithWhereUniqueInput = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: RecordCreateInput
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: RecordUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: RecordWhereUniqueInput
    
}


/**
 * **Record update many with where input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordUpdateManyWithWhereInput = {
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: RecordUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: RecordWhereInput
    
}


/**
 * **Record result**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordResult = {
    
    /**
     * **Created At**
     *
     * This synthesized field doesn't have a description.
     */
     createdAt: Date
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id: number
    
    /**
     * **Updated At**
     *
     * This synthesized field doesn't have a description.
     */
     updatedAt: Date
    
    /**
     * **Value**
     *
     * This synthesized field doesn't have a description.
     */
     value?: number
    
}
export type RecordResultGetPayload<S extends boolean | null | undefined | RecordArgs, U = keyof S> = S extends true
    ? RecordResult
    : S extends undefined
        ? never
        : S extends RecordArgs | RecordFindManyArgs
            ? 'include' extends U
                ? SelectSubset<Record, S> & {
                    [P in ExistKeys<S['include']>]:
                    never
                }
                : SelectSubset<RecordResult, S>
            : RecordResult

export type GetRecordAggregateType<T extends RecordAggregateArgs> = {
    [P in keyof T & keyof RecordAggregateResult]: P extends '_count' | 'count'
  ? T[P] extends true
    ? number
    : GetScalarType<T[P], RecordAggregateResult[P]>
  : GetScalarType<T[P], RecordAggregateResult[P]>
}

export type GetRecordGroupByPayload<T extends RecordGroupByArgs> =
  Array<
    PickEnumerable<RecordGroupByResult, T['by']> &
      {
        [P in ((keyof T) & (keyof RecordGroupByResult))]: P extends '_count'
          ? T[P] extends boolean
            ? number
            : GetScalarType<T[P], RecordGroupByResult[P]>
          : GetScalarType<T[P], RecordGroupByResult[P]>
      }
    >


/**
 * **Record count aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordCountAggregateResult = {
    
    /**
     * **All**
     *
     * This synthesized field doesn't have a description.
     */
     _all?: number
    
    /**
     * **Created At**
     *
     * This synthesized field doesn't have a description.
     */
     createdAt?: number
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Updated At**
     *
     * This synthesized field doesn't have a description.
     */
     updatedAt?: number
    
    /**
     * **Value**
     *
     * This synthesized field doesn't have a description.
     */
     value?: number
    
}


/**
 * **Record sum aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordSumAggregateResult = {
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
}


/**
 * **Record avg aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordAvgAggregateResult = {
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
}


/**
 * **Record min aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordMinAggregateResult = {
    
    /**
     * **Created At**
     *
     * This synthesized field doesn't have a description.
     */
     createdAt?: Date
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Updated At**
     *
     * This synthesized field doesn't have a description.
     */
     updatedAt?: Date
    
    /**
     * **Value**
     *
     * This synthesized field doesn't have a description.
     */
     value?: number
    
}


/**
 * **Record max aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordMaxAggregateResult = {
    
    /**
     * **Created At**
     *
     * This synthesized field doesn't have a description.
     */
     createdAt?: Date
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Updated At**
     *
     * This synthesized field doesn't have a description.
     */
     updatedAt?: Date
    
    /**
     * **Value**
     *
     * This synthesized field doesn't have a description.
     */
     value?: number
    
}


/**
 * **Record aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordAggregateResult = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: RecordAvgAggregateResult
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: RecordCountAggregateResult
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: RecordMaxAggregateResult
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: RecordMinAggregateResult
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: RecordSumAggregateResult
    
}


/**
 * **Record group by result**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordGroupByResult = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: RecordAvgAggregateResult
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: RecordCountAggregateResult
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: RecordMaxAggregateResult
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: RecordMinAggregateResult
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: RecordSumAggregateResult
    
    /**
     * **Created At**
     *
     * This synthesized field doesn't have a description.
     */
     createdAt?: Date
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Updated At**
     *
     * This synthesized field doesn't have a description.
     */
     updatedAt?: Date
    
    /**
     * **Value**
     *
     * This synthesized field doesn't have a description.
     */
     value?: number
    
}


/**
 * **Record args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: RecordInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: RecordSelect
    
}


/**
 * **Record find many args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordFindManyArgs = {
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: RecordWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: RecordSerializableScalarFields
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: RecordInclude
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<RecordOrderByInput>
    
    /**
     * **Page Number**
     *
     * This synthesized field doesn't have a description.
     */
     pageNumber?: number
    
    /**
     * **Page Size**
     *
     * This synthesized field doesn't have a description.
     */
     pageSize?: number
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: RecordSelect
    
    /**
     * **Skip**
     *
     * This synthesized field doesn't have a description.
     */
     skip?: number
    
    /**
     * **Take**
     *
     * This synthesized field doesn't have a description.
     */
     take?: number
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where?: RecordWhereInput
    
}


/**
 * **Record find first args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordFindFirstArgs = {
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: RecordWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: RecordSerializableScalarFields
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: RecordInclude
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<RecordOrderByInput>
    
    /**
     * **Page Number**
     *
     * This synthesized field doesn't have a description.
     */
     pageNumber?: number
    
    /**
     * **Page Size**
     *
     * This synthesized field doesn't have a description.
     */
     pageSize?: number
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: RecordSelect
    
    /**
     * **Skip**
     *
     * This synthesized field doesn't have a description.
     */
     skip?: number
    
    /**
     * **Take**
     *
     * This synthesized field doesn't have a description.
     */
     take?: number
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where?: RecordWhereInput
    
}


/**
 * **Record find unique args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordFindUniqueArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: RecordInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: RecordSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: RecordWhereUniqueInput
    
}


/**
 * **Record create args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordCreateArgs = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: RecordCreateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: RecordInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: RecordSelect
    
}


/**
 * **Record update args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordUpdateArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: RecordInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: RecordSelect
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: RecordUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: RecordWhereUniqueInput
    
}


/**
 * **Record upsert args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordUpsertArgs = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: RecordCreateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: RecordInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: RecordSelect
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: RecordUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: RecordWhereUniqueInput
    
}


/**
 * **Record copy args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordCopyArgs = {
    
    /**
     * **Copy**
     *
     * This synthesized field doesn't have a description.
     */
     copy: RecordUpdateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: RecordInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: RecordSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: RecordWhereUniqueInput
    
}


/**
 * **Record delete args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordDeleteArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: RecordInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: RecordSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: RecordWhereUniqueInput
    
}


/**
 * **Record create many args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordCreateManyArgs = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: Enumerable<RecordCreateInput>
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: RecordInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: RecordSelect
    
}


/**
 * **Record update many args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordUpdateManyArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: RecordInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: RecordSelect
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: RecordUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: RecordWhereInput
    
}


/**
 * **Record delete many args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordDeleteManyArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: RecordInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: RecordSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: RecordWhereInput
    
}


/**
 * **Record copy many args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordCopyManyArgs = {
    
    /**
     * **Copy**
     *
     * This synthesized field doesn't have a description.
     */
     copy: RecordUpdateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: RecordInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: RecordSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: RecordWhereInput
    
}


/**
 * **Record count args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordCountArgs = {
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: RecordWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: RecordSerializableScalarFields
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<RecordOrderByInput>
    
    /**
     * **Page Number**
     *
     * This synthesized field doesn't have a description.
     */
     pageNumber?: number
    
    /**
     * **Page Size**
     *
     * This synthesized field doesn't have a description.
     */
     pageSize?: number
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: RecordCountAggregateInputType
    
    /**
     * **Skip**
     *
     * This synthesized field doesn't have a description.
     */
     skip?: number
    
    /**
     * **Take**
     *
     * This synthesized field doesn't have a description.
     */
     take?: number
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where?: RecordWhereInput
    
}


/**
 * **Record aggregate args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordAggregateArgs = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: RecordAvgAggregateInputType
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: RecordCountAggregateInputType
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: RecordMaxAggregateInputType
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: RecordMinAggregateInputType
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: RecordSumAggregateInputType
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: RecordWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: RecordSerializableScalarFields
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<RecordOrderByInput>
    
    /**
     * **Page Number**
     *
     * This synthesized field doesn't have a description.
     */
     pageNumber?: number
    
    /**
     * **Page Size**
     *
     * This synthesized field doesn't have a description.
     */
     pageSize?: number
    
    /**
     * **Skip**
     *
     * This synthesized field doesn't have a description.
     */
     skip?: number
    
    /**
     * **Take**
     *
     * This synthesized field doesn't have a description.
     */
     take?: number
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where?: RecordWhereInput
    
}


/**
 * **Record group by args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordGroupByArgs = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: RecordAvgAggregateInputType
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: RecordCountAggregateInputType
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: RecordMaxAggregateInputType
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: RecordMinAggregateInputType
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: RecordSumAggregateInputType
    
    /**
     * **By**
     *
     * This synthesized field doesn't have a description.
     */
     by: Enumerable<RecordSerializableScalarFields>
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: RecordWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: RecordSerializableScalarFields
    
    /**
     * **Having**
     *
     * This synthesized field doesn't have a description.
     */
     having?: RecordScalarWhereWithAggregatesInput
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<RecordOrderByInput>
    
    /**
     * **Page Number**
     *
     * This synthesized field doesn't have a description.
     */
     pageNumber?: number
    
    /**
     * **Page Size**
     *
     * This synthesized field doesn't have a description.
     */
     pageSize?: number
    
    /**
     * **Skip**
     *
     * This synthesized field doesn't have a description.
     */
     skip?: number
    
    /**
     * **Take**
     *
     * This synthesized field doesn't have a description.
     */
     take?: number
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where?: RecordWhereInput
    
}


/**
 * **Record scalar update input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordScalarUpdateInput = {
    
    /**
     * **Created At**
     *
     * This synthesized field doesn't have a description.
     */
     createdAt?: Date
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Updated At**
     *
     * This synthesized field doesn't have a description.
     */
     updatedAt?: Date
    
    /**
     * **Value**
     *
     * This synthesized field doesn't have a description.
     */
     value?: number
    
}


/**
 * **Record sign in checker ids**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordSignInCheckerIds = {
    
}


/**
 * **Record sign in checker companions**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordSignInCheckerCompanions = {
    
}


/**
 * **Record sign in input**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordSignInInput = {
    
    /**
     * **Credentials**
     *
     * This synthesized field doesn't have a description.
     */
     credentials: RecordSignInArgs
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: RecordInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: RecordSelect
    
}


/**
 * **Record sign in args**
 *
 * This synthesized interface doesn't have a description
 */
export type RecordSignInArgs = {
    
}



export namespace std {


    /**
     * **Sort Order**
     *
     * Represents the sort order
     */
    export type Sort = "asc" | "desc"

    /**
     * **String Match Mode**
     *
     * Whether the string query is case sensitive or not
     */
    export type StringMatchMode = "default" | "caseInsensitive"

    /// ## Sort Order
    ///
    /// Represents the sort order
    export const enum SortEnumType {

        /// ### Asc
        ///
        /// This enum member doesn't have a description.
        asc = "asc",

        /// ### Desc
        ///
        /// This enum member doesn't have a description.
        desc = "desc",
    }

    /// ## String Match Mode
    ///
    /// Whether the string query is case sensitive or not
    export const enum StringMatchModeEnumType {

        /// ### Default
        ///
        /// This enum member doesn't have a description.
        default = "default",

        /// ### Case insensitive
        ///
        /// This enum member doesn't have a description.
        caseInsensitive = "caseInsensitive",
    }


    /**
     * **Empty**
     *
     * The empty interface
     */
    export type Empty = {
        
    }


    /**
     * **Data**
     *
     * This interface is common for action output
     */
    export type Data<T> = {
        
        /**
         * **Data**
         *
         * This interface field doesn't have a description.
         */
         data: T
        
    }


    /**
     * **Data and Meta**
     *
     * This interface is common for action output with meta information
     */
    export type DataMeta<T, U> = {
        
        /**
         * **Data**
         *
         * This interface field doesn't have a description.
         */
         data: T
        
        /**
         * **Meta**
         *
         * This interface field doesn't have a description.
         */
         meta: U
        
    }


    /**
     * **Paging info**
     *
     * This interface doesn't have a description.
     */
    export type PagingInfo = {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         count: number
        
        /**
         * **Number of pages**
         *
         * This interface field doesn't have a description.
         */
         numberOfPages?: number
        
    }


    /**
     * **Response error**
     *
     * This interface doesn't have a description.
     */
    export type ResponseError = {
        
        /**
         * **Type**
         *
         * This interface field doesn't have a description.
         */
         type: string
        
        /**
         * **Message**
         *
         * This interface field doesn't have a description.
         */
         message: string
        
        /**
         * **Fields**
         *
         * This interface field doesn't have a description.
         */
         fields: {[key: string]: string} | null
        
    }


    /**
     * **Bool filter**
     *
     * This interface doesn't have a description.
     */
    export type BoolFilter = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: boolean
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: boolean | std.BoolFilter
        
    }


    /**
     * **Bool nullable filter**
     *
     * This interface doesn't have a description.
     */
    export type BoolNullableFilter = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: boolean | null
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: boolean | null | std.BoolNullableFilter
        
    }


    /**
     * **Filter**
     *
     * This interface doesn't have a description.
     */
    export type Filter<T> = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: T
        
        /**
         * **In**
         *
         * This interface field doesn't have a description.
         */
         in?: T[]
        
        /**
         * **Not in**
         *
         * This interface field doesn't have a description.
         */
         notIn?: T[]
        
        /**
         * **Lt**
         *
         * This interface field doesn't have a description.
         */
         lt?: T
        
        /**
         * **Lte**
         *
         * This interface field doesn't have a description.
         */
         lte?: T
        
        /**
         * **Gt**
         *
         * This interface field doesn't have a description.
         */
         gt?: T
        
        /**
         * **Gte**
         *
         * This interface field doesn't have a description.
         */
         gte?: T
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: T | std.Filter<T>
        
    }


    /**
     * **Nullable filter**
     *
     * This interface doesn't have a description.
     */
    export type NullableFilter<T> = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: T | null
        
        /**
         * **In**
         *
         * This interface field doesn't have a description.
         */
         in?: (T | null)[]
        
        /**
         * **Not in**
         *
         * This interface field doesn't have a description.
         */
         notIn?: (T | null)[]
        
        /**
         * **Lt**
         *
         * This interface field doesn't have a description.
         */
         lt?: T
        
        /**
         * **Lte**
         *
         * This interface field doesn't have a description.
         */
         lte?: T
        
        /**
         * **Gt**
         *
         * This interface field doesn't have a description.
         */
         gt?: T
        
        /**
         * **Gte**
         *
         * This interface field doesn't have a description.
         */
         gte?: T
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: T | null | std.NullableFilter<T>
        
    }


    /**
     * **String filter**
     *
     * This interface doesn't have a description.
     */
    export type StringFilter = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: string
        
        /**
         * **In**
         *
         * This interface field doesn't have a description.
         */
         in?: string[]
        
        /**
         * **Not in**
         *
         * This interface field doesn't have a description.
         */
         notIn?: string[]
        
        /**
         * **Lt**
         *
         * This interface field doesn't have a description.
         */
         lt?: string
        
        /**
         * **Lte**
         *
         * This interface field doesn't have a description.
         */
         lte?: string
        
        /**
         * **Gt**
         *
         * This interface field doesn't have a description.
         */
         gt?: string
        
        /**
         * **Gte**
         *
         * This interface field doesn't have a description.
         */
         gte?: string
        
        /**
         * **Contains**
         *
         * This interface field doesn't have a description.
         */
         contains?: string
        
        /**
         * **Starts with**
         *
         * This interface field doesn't have a description.
         */
         startsWith?: string
        
        /**
         * **Ends with**
         *
         * This interface field doesn't have a description.
         */
         endsWith?: string
        
        /**
         * **Matches**
         *
         * This interface field doesn't have a description.
         */
         matches?: string
        
        /**
         * **Mode**
         *
         * This interface field doesn't have a description.
         */
         mode?: std.StringMatchMode
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: string | std.StringFilter
        
    }


    /**
     * **String nullable filter**
     *
     * This interface doesn't have a description.
     */
    export type StringNullableFilter = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: string | null
        
        /**
         * **In**
         *
         * This interface field doesn't have a description.
         */
         in?: (string | null)[]
        
        /**
         * **Not in**
         *
         * This interface field doesn't have a description.
         */
         notIn?: (string | null)[]
        
        /**
         * **Lt**
         *
         * This interface field doesn't have a description.
         */
         lt?: string
        
        /**
         * **Lte**
         *
         * This interface field doesn't have a description.
         */
         lte?: string
        
        /**
         * **Gt**
         *
         * This interface field doesn't have a description.
         */
         gt?: string
        
        /**
         * **Gte**
         *
         * This interface field doesn't have a description.
         */
         gte?: string
        
        /**
         * **Contains**
         *
         * This interface field doesn't have a description.
         */
         contains?: string
        
        /**
         * **Starts with**
         *
         * This interface field doesn't have a description.
         */
         startsWith?: string
        
        /**
         * **Ends with**
         *
         * This interface field doesn't have a description.
         */
         endsWith?: string
        
        /**
         * **Matches**
         *
         * This interface field doesn't have a description.
         */
         matches?: string
        
        /**
         * **Mode**
         *
         * This interface field doesn't have a description.
         */
         mode?: std.StringMatchMode
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: string | null | std.StringNullableFilter
        
    }


    /**
     * **Enum filter**
     *
     * This interface doesn't have a description.
     */
    export type EnumFilter<T> = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: T
        
        /**
         * **In**
         *
         * This interface field doesn't have a description.
         */
         in?: T[]
        
        /**
         * **Not in**
         *
         * This interface field doesn't have a description.
         */
         notIn?: T[]
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: T | std.EnumFilter<T>
        
    }


    /**
     * **Enum nullable filter**
     *
     * This interface doesn't have a description.
     */
    export type EnumNullableFilter<T> = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: T | null
        
        /**
         * **In**
         *
         * This interface field doesn't have a description.
         */
         in?: (T | null)[]
        
        /**
         * **Not in**
         *
         * This interface field doesn't have a description.
         */
         notIn?: (T | null)[]
        
        /**
         * **Not**
         *
         * This interface field doesn't have a description.
         */
         not?: T | null | std.EnumNullableFilter<T>
        
    }


    /**
     * **Array filter**
     *
     * This interface doesn't have a description.
     */
    export type ArrayFilter<T> = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: T[]
        
        /**
         * **Has**
         *
         * This interface field doesn't have a description.
         */
         has?: T
        
        /**
         * **Has some**
         *
         * This interface field doesn't have a description.
         */
         hasSome?: T[]
        
        /**
         * **Has every**
         *
         * This interface field doesn't have a description.
         */
         hasEvery?: T[]
        
        /**
         * **Is empty**
         *
         * This interface field doesn't have a description.
         */
         isEmpty?: boolean
        
        /**
         * **Length**
         *
         * This interface field doesn't have a description.
         */
         length?: number
        
    }


    /**
     * **Array nullable filter**
     *
     * This interface doesn't have a description.
     */
    export type ArrayNullableFilter<T> = {
        
        /**
         * **Equals**
         *
         * This interface field doesn't have a description.
         */
         equals?: T[] | null
        
        /**
         * **Has**
         *
         * This interface field doesn't have a description.
         */
         has?: T
        
        /**
         * **Has some**
         *
         * This interface field doesn't have a description.
         */
         hasSome?: T[]
        
        /**
         * **Has every**
         *
         * This interface field doesn't have a description.
         */
         hasEvery?: T[]
        
        /**
         * **Is empty**
         *
         * This interface field doesn't have a description.
         */
         isEmpty?: boolean
        
        /**
         * **Length**
         *
         * This interface field doesn't have a description.
         */
         length?: number
        
    }


    /**
     * **Bool with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type BoolWithAggregatesFilter = std.BoolFilter & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.BoolFilter
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.BoolFilter
        
    }


    /**
     * **Bool nullable with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type BoolNullableWithAggregatesFilter = std.BoolNullableFilter & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.BoolNullableFilter
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.BoolNullableFilter
        
    }


    /**
     * **Int number with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type IntNumberWithAggregatesFilter<T> = std.Filter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.Filter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.Filter<T>
        
        /**
         * **Avg**
         *
         * This interface field doesn't have a description.
         */
         _avg?: std.Filter<number>
        
        /**
         * **Sum**
         *
         * This interface field doesn't have a description.
         */
         _sum?: std.Filter<number>
        
    }


    /**
     * **Int number nullable with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type IntNumberNullableWithAggregatesFilter<T> = std.NullableFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.NullableFilter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.NullableFilter<T>
        
        /**
         * **Avg**
         *
         * This interface field doesn't have a description.
         */
         _avg?: std.NullableFilter<number>
        
        /**
         * **Sum**
         *
         * This interface field doesn't have a description.
         */
         _sum?: std.NullableFilter<number>
        
    }


    /**
     * **Float number with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type FloatNumberWithAggregatesFilter<T> = std.Filter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.Filter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.Filter<T>
        
        /**
         * **Avg**
         *
         * This interface field doesn't have a description.
         */
         _avg?: std.Filter<number>
        
        /**
         * **Sum**
         *
         * This interface field doesn't have a description.
         */
         _sum?: std.Filter<number>
        
    }


    /**
     * **Float number nullable with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type FloatNumberNullableWithAggregatesFilter<T> = std.NullableFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.NullableFilter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.NullableFilter<T>
        
        /**
         * **Avg**
         *
         * This interface field doesn't have a description.
         */
         _avg?: std.NullableFilter<number>
        
        /**
         * **Sum**
         *
         * This interface field doesn't have a description.
         */
         _sum?: std.NullableFilter<number>
        
    }


    /**
     * **Decimal with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type DecimalWithAggregatesFilter = std.Filter<Decimal> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.Filter<Decimal>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.Filter<Decimal>
        
        /**
         * **Avg**
         *
         * This interface field doesn't have a description.
         */
         _avg?: std.Filter<Decimal>
        
        /**
         * **Sum**
         *
         * This interface field doesn't have a description.
         */
         _sum?: std.Filter<Decimal>
        
    }


    /**
     * **Decimal nullable with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type DecimalNullableWithAggregatesFilter<T> = std.NullableFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.NullableFilter<Decimal>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.NullableFilter<Decimal>
        
        /**
         * **Avg**
         *
         * This interface field doesn't have a description.
         */
         _avg?: std.NullableFilter<Decimal>
        
        /**
         * **Sum**
         *
         * This interface field doesn't have a description.
         */
         _sum?: std.NullableFilter<Decimal>
        
    }


    /**
     * **Aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type AggregatesFilter<T> = std.Filter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.Filter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.Filter<T>
        
    }


    /**
     * **Nullable aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type NullableAggregatesFilter<T> = std.NullableFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.NullableFilter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.NullableFilter<T>
        
    }


    /**
     * **String with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type StringWithAggregatesFilter = std.StringFilter & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.StringFilter
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.StringFilter
        
    }


    /**
     * **String nullable with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type StringNullableWithAggregatesFilter = std.StringNullableFilter & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.StringNullableFilter
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.StringNullableFilter
        
    }


    /**
     * **Enum with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type EnumWithAggregatesFilter<T> = std.EnumFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.EnumFilter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.EnumFilter<T>
        
    }


    /**
     * **Enum nullable with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type EnumNullableWithAggregatesFilter<T> = std.EnumNullableFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.EnumNullableFilter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.EnumNullableFilter<T>
        
    }


    /**
     * **Array with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type ArrayWithAggregatesFilter<T> = std.ArrayFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.ArrayFilter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.ArrayFilter<T>
        
    }


    /**
     * **Array nullable with aggregates filter**
     *
     * This interface doesn't have a description.
     */
    export type ArrayNullableWithAggregatesFilter<T> = std.ArrayNullableFilter<T> & {
        
        /**
         * **Count**
         *
         * This interface field doesn't have a description.
         */
         _count?: number
        
        /**
         * **Min**
         *
         * This interface field doesn't have a description.
         */
         _min?: std.ArrayNullableFilter<T>
        
        /**
         * **Max**
         *
         * This interface field doesn't have a description.
         */
         _max?: std.ArrayNullableFilter<T>
        
    }


    /**
     * **Number atomic update operation input**
     *
     * This interface doesn't have a description.
     */
    export type NumberAtomicUpdateOperationInput<T> = {
        
        /**
         * **Increment**
         *
         * This interface field doesn't have a description.
         */
         increment?: T
        
        /**
         * **Decrement**
         *
         * This interface field doesn't have a description.
         */
         decrement?: T
        
        /**
         * **Multiply**
         *
         * This interface field doesn't have a description.
         */
         multiply?: T
        
        /**
         * **Divide**
         *
         * This interface field doesn't have a description.
         */
         divide?: T
        
    }


    /**
     * **Array atomic update operation input**
     *
     * This interface doesn't have a description.
     */
    export type ArrayAtomicUpdateOperationInput<T> = {
        
        /**
         * **Push**
         *
         * This interface field doesn't have a description.
         */
         push?: T
        
    }



    export namespace admin {










        declare class AdminNamespace {

        }

    }

    export namespace bcrypt {










        declare class BcryptNamespace {

        }

    }

    export namespace identity {



        /**
         * **Token info**
         *
         * This interface doesn't have a description.
         */
        export type TokenInfo = {
            
            /**
             * **Token**
             *
             * This interface field doesn't have a description.
             */
             token: string
            
        }









        declare class IdentityNamespace {

        }

    }










}






export class RecordModel {
    findManyObjects(query: RecordFindManyArgs): Promise<Record[]>
    findUniqueObject(query: RecordFindUniqueArgs): Promise<Record | null>
    findFirstObject(query: RecordFindManyArgs): Promise<Record | null>
    createObject(input?: RecordCreateInput): Promise<Record>
    count(input?: RecordCountArgs): Promise<number>
    aggregate<T extends RecordAggregateArgs>(input?: Subset<T, RecordAggregateArgs>): Promise<GetRecordAggregateType<T>>
    groupBy<T extends RecordGroupByArgs,
      HasSelectOrTake extends Or<
        Extends<'skip', Keys<T>>,
        Extends<'take', Keys<T>>
      >,
      OrderByArg extends True extends HasSelectOrTake
        ? { orderBy: RecordGroupByArgs['orderBy'] }
        : { orderBy?: RecordGroupByArgs['orderBy'] },
      OrderFields extends ExcludeUnderscoreKeys<Keys<MaybeTupleToUnion<T['orderBy']>>>,
      ByFields extends MaybeTupleToUnion<T['by']>,
      ByValid extends Has<ByFields, OrderFields>,
      HavingFields extends GetHavingFields<T['having']>,
      HavingValid extends Has<ByFields, HavingFields>,
      ByEmpty extends T['by'] extends never[] ? True : False,
      InputErrors extends ByEmpty extends True
      ? `Error: "by" must not be empty.`
      : HavingValid extends False
      ? {
          [P in HavingFields]: P extends ByFields
            ? never
            : P extends string
            ? `Error: Field "${P}" used in "having" needs to be provided in "by".`
            : [
                Error,
                'Field ',
                P,
                ` in "having" needs to be provided in "by"`,
              ]
        }[HavingFields]
      : 'take' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "take", you also need to provide "orderBy"'
      : 'skip' extends Keys<T>
      ? 'orderBy' extends Keys<T>
        ? ByValid extends True
          ? {}
          : {
              [P in OrderFields]: P extends ByFields
                ? never
                : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
            }[OrderFields]
        : 'Error: If you provide "skip", you also need to provide "orderBy"'
      : ByValid extends True
      ? {}
      : {
          [P in OrderFields]: P extends ByFields
            ? never
            : `Error: Field "${P}" in "orderBy" needs to be provided in "by"`
        }[OrderFields]>(input: SubsetIntersection<T, RecordGroupByArgs, OrderByArg> & InputErrors): Promise<{} extends InputErrors ? GetRecordGroupByPayload<T> : InputErrors>
    
    sql<T>(sql: string): Promise<T[]>
    
}

export class Record {
    get isNew(): boolean
    get isModified(): boolean
    set(input: RecordUpdateInput): Promise<void>
    update(input: RecordScalarUpdateInput): Promise<void>
    save(): Promise<void>
    delete(): Promise<void>
    toTeon(): Promise<RecordResult>
    /// ## Id
    ///
    /// This field doesn't have a description.
    get id(): number

    /// ## Id
    ///
    /// This field doesn't have a description.
    set id(newValue: number)
    /// ## Value
    ///
    /// This field doesn't have a description.
    get value(): number | null

    /// ## Value
    ///
    /// This field doesn't have a description.
    set value(newValue: number | null)
    /// ## Created at
    ///
    /// This field doesn't have a description.
    get createdAt(): Date

    /// ## Created at
    ///
    /// This field doesn't have a description.
    set createdAt(newValue: Date)
    /// ## Updated at
    ///
    /// This field doesn't have a description.
    get updatedAt(): Date

    /// ## Updated at
    ///
    /// This field doesn't have a description.
    set updatedAt(newValue: Date)
}



type EchoPathArguments = {

    data: string

}

type StaticPathArguments = {

    path: string

}

declare class Teo {

    transaction(callback: (teo: Teo) => Promise<void>): Promise<void>

    get record(): RecordModel
}
