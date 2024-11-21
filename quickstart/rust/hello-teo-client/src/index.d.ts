import Decimal from "decimal.js"

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

export declare class TeoError extends Error {
    type: string

    errors: {[key: string]: string} | null

    constructor(responseError: std.ResponseError)

    get name(): string
}

/**
 * **Post scalar fields**
 *
 * This synthesized enum doesn't have a description.
 */
export type PostScalarFields = "authorId" | "content" | "id" | "published" | "title"

/**
 * **Post serializable scalar fields**
 *
 * This synthesized enum doesn't have a description.
 */
export type PostSerializableScalarFields = "authorId" | "content" | "id" | "published" | "title"

/**
 * **Post relations**
 *
 * This synthesized enum doesn't have a description.
 */
export type PostRelations = "author"

/**
 * **Post direct relations**
 *
 * This synthesized enum doesn't have a description.
 */
export type PostDirectRelations = "author"

/**
 * **Post indirect relations**
 *
 * This synthesized enum doesn't have a description.
 */
export type PostIndirectRelations = undefined

/**
 * **User scalar fields**
 *
 * This synthesized enum doesn't have a description.
 */
export type UserScalarFields = "email" | "id" | "name"

/**
 * **User serializable scalar fields**
 *
 * This synthesized enum doesn't have a description.
 */
export type UserSerializableScalarFields = "email" | "id" | "name"

/**
 * **User relations**
 *
 * This synthesized enum doesn't have a description.
 */
export type UserRelations = "posts"

/**
 * **User direct relations**
 *
 * This synthesized enum doesn't have a description.
 */
export type UserDirectRelations = "posts"

/**
 * **User indirect relations**
 *
 * This synthesized enum doesn't have a description.
 */
export type UserIndirectRelations = undefined

/// ## Post scalar fields
///
/// This synthesized enum doesn't have a description.
export const enum PostScalarFieldsEnumType {

    /// ### Author id
    ///
    /// This synthesized enum member doesn't have a description.
    authorId = "authorId",

    /// ### Content
    ///
    /// This synthesized enum member doesn't have a description.
    content = "content",

    /// ### Id
    ///
    /// This synthesized enum member doesn't have a description.
    id = "id",

    /// ### Published
    ///
    /// This synthesized enum member doesn't have a description.
    published = "published",

    /// ### Title
    ///
    /// This synthesized enum member doesn't have a description.
    title = "title",
}

/// ## Post serializable scalar fields
///
/// This synthesized enum doesn't have a description.
export const enum PostSerializableScalarFieldsEnumType {

    /// ### Author id
    ///
    /// This synthesized enum member doesn't have a description.
    authorId = "authorId",

    /// ### Content
    ///
    /// This synthesized enum member doesn't have a description.
    content = "content",

    /// ### Id
    ///
    /// This synthesized enum member doesn't have a description.
    id = "id",

    /// ### Published
    ///
    /// This synthesized enum member doesn't have a description.
    published = "published",

    /// ### Title
    ///
    /// This synthesized enum member doesn't have a description.
    title = "title",
}

/// ## Post relations
///
/// This synthesized enum doesn't have a description.
export const enum PostRelationsEnumType {

    /// ### Author
    ///
    /// This synthesized enum member doesn't have a description.
    author = "author",
}

/// ## Post direct relations
///
/// This synthesized enum doesn't have a description.
export const enum PostDirectRelationsEnumType {

    /// ### Author
    ///
    /// This synthesized enum member doesn't have a description.
    author = "author",
}

/// ## Post indirect relations
///
/// This synthesized enum doesn't have a description.
export const enum PostIndirectRelationsEnumType {
}

/// ## User scalar fields
///
/// This synthesized enum doesn't have a description.
export const enum UserScalarFieldsEnumType {

    /// ### Email
    ///
    /// This synthesized enum member doesn't have a description.
    email = "email",

    /// ### Id
    ///
    /// This synthesized enum member doesn't have a description.
    id = "id",

    /// ### Name
    ///
    /// This synthesized enum member doesn't have a description.
    name = "name",
}

/// ## User serializable scalar fields
///
/// This synthesized enum doesn't have a description.
export const enum UserSerializableScalarFieldsEnumType {

    /// ### Email
    ///
    /// This synthesized enum member doesn't have a description.
    email = "email",

    /// ### Id
    ///
    /// This synthesized enum member doesn't have a description.
    id = "id",

    /// ### Name
    ///
    /// This synthesized enum member doesn't have a description.
    name = "name",
}

/// ## User relations
///
/// This synthesized enum doesn't have a description.
export const enum UserRelationsEnumType {

    /// ### Posts
    ///
    /// This synthesized enum member doesn't have a description.
    posts = "posts",
}

/// ## User direct relations
///
/// This synthesized enum doesn't have a description.
export const enum UserDirectRelationsEnumType {

    /// ### Posts
    ///
    /// This synthesized enum member doesn't have a description.
    posts = "posts",
}

/// ## User indirect relations
///
/// This synthesized enum doesn't have a description.
export const enum UserIndirectRelationsEnumType {
}


/**
 * **Post select**
 *
 * This synthesized interface doesn't have a description
 */
export type PostSelect = {
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: boolean
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: boolean
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title?: boolean
    
}


/**
 * **Post include**
 *
 * This synthesized interface doesn't have a description
 */
export type PostInclude = {
    
    /**
     * **Author**
     *
     * This synthesized field doesn't have a description.
     */
     author?: UserArgs | boolean
    
}


/**
 * **Post where input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostWhereInput = {
    
    /**
     * **And**
     *
     * This synthesized field doesn't have a description.
     */
     AND?: PostWhereInput[]
    
    /**
     * **Not**
     *
     * This synthesized field doesn't have a description.
     */
     NOT?: PostWhereInput
    
    /**
     * **Or**
     *
     * This synthesized field doesn't have a description.
     */
     OR?: PostWhereInput[]
    
    /**
     * **Author**
     *
     * This synthesized field doesn't have a description.
     */
     author?: UserRelationFilter
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: number | std.Filter<number>
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: string | null | std.StringNullableFilter
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number | std.Filter<number>
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: boolean | std.BoolFilter
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title?: string | std.StringFilter
    
}


/**
 * **Post where unique input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostWhereUniqueInput = {
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id: number
    
}


/**
 * **Post scalar where with aggregates input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostScalarWhereWithAggregatesInput = {
    
    /**
     * **And**
     *
     * This synthesized field doesn't have a description.
     */
     AND?: PostWhereInput[]
    
    /**
     * **Not**
     *
     * This synthesized field doesn't have a description.
     */
     NOT?: PostWhereInput
    
    /**
     * **Or**
     *
     * This synthesized field doesn't have a description.
     */
     OR?: PostWhereInput[]
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: number | std.IntNumberWithAggregatesFilter<number>
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: string | null | std.StringNullableWithAggregatesFilter
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number | std.IntNumberWithAggregatesFilter<number>
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: boolean | std.BoolWithAggregatesFilter
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title?: string | std.StringWithAggregatesFilter
    
}


/**
 * **Post relation filter**
 *
 * This synthesized interface doesn't have a description
 */
export type PostRelationFilter = {
    
    /**
     * **Is**
     *
     * This synthesized field doesn't have a description.
     */
     is?: PostWhereInput
    
    /**
     * **Is Not**
     *
     * This synthesized field doesn't have a description.
     */
     isNot?: PostWhereInput
    
}


/**
 * **Post list relation filter**
 *
 * This synthesized interface doesn't have a description
 */
export type PostListRelationFilter = {
    
    /**
     * **Every**
     *
     * This synthesized field doesn't have a description.
     */
     every?: PostWhereInput
    
    /**
     * **None**
     *
     * This synthesized field doesn't have a description.
     */
     none?: PostWhereInput
    
    /**
     * **Some**
     *
     * This synthesized field doesn't have a description.
     */
     some?: PostWhereInput
    
}


/**
 * **Post order by input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostOrderByInput = {
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: std.Sort
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: std.Sort
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: std.Sort
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: std.Sort
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title?: std.Sort
    
}


/**
 * **Post count aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type PostCountAggregateInputType = {
    
    /**
     * **All**
     *
     * This synthesized field doesn't have a description.
     */
     _all?: boolean
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: boolean
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: boolean
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title?: boolean
    
}


/**
 * **Post sum aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type PostSumAggregateInputType = {
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
}


/**
 * **Post avg aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type PostAvgAggregateInputType = {
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
}


/**
 * **Post min aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type PostMinAggregateInputType = {
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: boolean
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: boolean
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title?: boolean
    
}


/**
 * **Post max aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type PostMaxAggregateInputType = {
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: boolean
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: boolean
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title?: boolean
    
}


/**
 * **Post create input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostCreateInput = {
    
    /**
     * **Author**
     *
     * This synthesized field doesn't have a description.
     */
     author?: UserCreateNestedOneWithoutPostsInput
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: number
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: string
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: boolean
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title: string
    
}


/**
 * **Post create without author input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostCreateWithoutAuthorInput = {
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: number
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: string
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: boolean
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title: string
    
}


/**
 * **Post update input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostUpdateInput = {
    
    /**
     * **Author**
     *
     * This synthesized field doesn't have a description.
     */
     author?: UserUpdateNestedOneWithoutPostsInput
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: number
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: string
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: boolean
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title?: string
    
}


/**
 * **Post update without author input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostUpdateWithoutAuthorInput = {
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: number
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: string
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: boolean
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title?: string
    
}


/**
 * **Post create nested one input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostCreateNestedOneInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: PostWhereUniqueInput
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: PostConnectOrCreateInput
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: PostCreateInput
    
}


/**
 * **Post create nested one without author input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostCreateNestedOneWithoutAuthorInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: PostWhereUniqueInput
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: PostConnectOrCreateWithoutAuthorInput
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: PostCreateInput
    
}


/**
 * **Post create nested many input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostCreateNestedManyInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: Enumerable<PostWhereUniqueInput>
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: Enumerable<PostConnectOrCreateInput>
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: Enumerable<PostCreateInput>
    
}


/**
 * **Post create nested many without author input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostCreateNestedManyWithoutAuthorInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: Enumerable<PostWhereUniqueInput>
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: Enumerable<PostConnectOrCreateWithoutAuthorInput>
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: Enumerable<PostCreateInput>
    
}


/**
 * **Post update nested one input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostUpdateNestedOneInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: PostWhereUniqueInput
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: PostConnectOrCreateInput
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: PostCreateInput
    
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
     set?: PostWhereUniqueInput
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update?: PostUpdateInput
    
    /**
     * **Upsert**
     *
     * This synthesized field doesn't have a description.
     */
     upsert?: PostUpsertWithWhereUniqueInput
    
}


/**
 * **Post update nested one without author input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostUpdateNestedOneWithoutAuthorInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: PostWhereUniqueInput
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: PostConnectOrCreateWithoutAuthorInput
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: PostCreateInput
    
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
     set?: PostWhereUniqueInput
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update?: PostUpdateInput
    
    /**
     * **Upsert**
     *
     * This synthesized field doesn't have a description.
     */
     upsert?: PostUpsertWithWhereUniqueWithoutAuthorInput
    
}


/**
 * **Post update nested many input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostUpdateNestedManyInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: Enumerable<PostWhereUniqueInput>
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: Enumerable<PostConnectOrCreateInput>
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: Enumerable<PostCreateInput>
    
    /**
     * **Delete**
     *
     * This synthesized field doesn't have a description.
     */
     delete?: Enumerable<PostWhereUniqueInput>
    
    /**
     * **Delete Many**
     *
     * This synthesized field doesn't have a description.
     */
     deleteMany?: Enumerable<PostWhereInput>
    
    /**
     * **Disconnect**
     *
     * This synthesized field doesn't have a description.
     */
     disconnect?: Enumerable<PostWhereUniqueInput>
    
    /**
     * **Set**
     *
     * This synthesized field doesn't have a description.
     */
     set?: Enumerable<PostWhereUniqueInput>
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update?: Enumerable<PostUpdateWithWhereUniqueInput>
    
    /**
     * **Update Many**
     *
     * This synthesized field doesn't have a description.
     */
     updateMany?: Enumerable<PostUpdateManyWithWhereInput>
    
    /**
     * **Upsert**
     *
     * This synthesized field doesn't have a description.
     */
     upsert?: Enumerable<PostUpsertWithWhereUniqueInput>
    
}


/**
 * **Post update nested many without author input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostUpdateNestedManyWithoutAuthorInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: Enumerable<PostWhereUniqueInput>
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: Enumerable<PostConnectOrCreateWithoutAuthorInput>
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: Enumerable<PostCreateInput>
    
    /**
     * **Delete**
     *
     * This synthesized field doesn't have a description.
     */
     delete?: Enumerable<PostWhereUniqueInput>
    
    /**
     * **Delete Many**
     *
     * This synthesized field doesn't have a description.
     */
     deleteMany?: Enumerable<PostWhereInput>
    
    /**
     * **Disconnect**
     *
     * This synthesized field doesn't have a description.
     */
     disconnect?: Enumerable<PostWhereUniqueInput>
    
    /**
     * **Set**
     *
     * This synthesized field doesn't have a description.
     */
     set?: Enumerable<PostWhereUniqueInput>
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update?: Enumerable<PostUpdateWithWhereUniqueWithoutAuthorInput>
    
    /**
     * **Update Many**
     *
     * This synthesized field doesn't have a description.
     */
     updateMany?: Enumerable<PostUpdateManyWithWhereWithoutAuthorInput>
    
    /**
     * **Upsert**
     *
     * This synthesized field doesn't have a description.
     */
     upsert?: Enumerable<PostUpsertWithWhereUniqueWithoutAuthorInput>
    
}


/**
 * **Post connect or create input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostConnectOrCreateInput = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: PostCreateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereUniqueInput
    
}


/**
 * **Post connect or create without author input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostConnectOrCreateWithoutAuthorInput = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: PostCreateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereUniqueInput
    
}


/**
 * **Post update with where unique input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostUpdateWithWhereUniqueInput = {
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: PostUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereUniqueInput
    
}


/**
 * **Post update with where unique without author input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostUpdateWithWhereUniqueWithoutAuthorInput = {
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: PostUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereUniqueInput
    
}


/**
 * **Post upsert with where unique input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostUpsertWithWhereUniqueInput = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: PostCreateInput
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: PostUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereUniqueInput
    
}


/**
 * **Post upsert with where unique without author input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostUpsertWithWhereUniqueWithoutAuthorInput = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: PostCreateInput
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: PostUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereUniqueInput
    
}


/**
 * **Post update many with where input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostUpdateManyWithWhereInput = {
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: PostUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereInput
    
}


/**
 * **Post update many with where without author input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostUpdateManyWithWhereWithoutAuthorInput = {
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: PostUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereInput
    
}


/**
 * **Post**
 *
 * This synthesized interface doesn't have a description
 */
export type Post = {
    
    /**
     * **Author**
     *
     * This synthesized field doesn't have a description.
     */
     author: User
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId: number
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: string
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id: number
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published: boolean
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title: string
    
}
export type PostGetPayload<S extends boolean | null | undefined | PostArgs, U = keyof S> = S extends true
    ? Post
    : S extends undefined
        ? never
        : S extends PostArgs | PostFindManyArgs
            ? 'include' extends U
                ? SelectSubset<Post, S> & {
                    [P in ExistKeys<S['include']>]:
                        P extends 'author' ? UserGetPayload<S['include'][P]> :
                    never
                }
                : SelectSubset<Post, S>
            : Post

export type GetPostAggregateType<T extends PostAggregateArgs> = {
    [P in keyof T & keyof PostAggregateResult]: P extends '_count' | 'count'
  ? T[P] extends true
    ? number
    : GetScalarType<T[P], PostAggregateResult[P]>
  : GetScalarType<T[P], PostAggregateResult[P]>
}

export type GetPostGroupByPayload<T extends PostGroupByArgs> =
  Array<
    PickEnumerable<PostGroupByResult, T['by']> &
      {
        [P in ((keyof T) & (keyof PostGroupByResult))]: P extends '_count'
          ? T[P] extends boolean
            ? number
            : GetScalarType<T[P], PostGroupByResult[P]>
          : GetScalarType<T[P], PostGroupByResult[P]>
      }
    >


/**
 * **Post count aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type PostCountAggregateResult = {
    
    /**
     * **All**
     *
     * This synthesized field doesn't have a description.
     */
     _all?: number
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: number
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: number
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: number
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title?: number
    
}


/**
 * **Post sum aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type PostSumAggregateResult = {
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: number
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
}


/**
 * **Post avg aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type PostAvgAggregateResult = {
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: number
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
}


/**
 * **Post min aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type PostMinAggregateResult = {
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: number
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: string
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: boolean
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title?: string
    
}


/**
 * **Post max aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type PostMaxAggregateResult = {
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: number
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: string
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: boolean
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title?: string
    
}


/**
 * **Post aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type PostAggregateResult = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: PostAvgAggregateResult
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: PostCountAggregateResult
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: PostMaxAggregateResult
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: PostMinAggregateResult
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: PostSumAggregateResult
    
}


/**
 * **Post group by result**
 *
 * This synthesized interface doesn't have a description
 */
export type PostGroupByResult = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: PostAvgAggregateResult
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: PostCountAggregateResult
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: PostMaxAggregateResult
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: PostMinAggregateResult
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: PostSumAggregateResult
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: number
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: string
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: boolean
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title?: string
    
}


/**
 * **Post args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: PostInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: PostSelect
    
}


/**
 * **Post find many args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostFindManyArgs = {
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: PostWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: PostSerializableScalarFields
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: PostInclude
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<PostOrderByInput>
    
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
     select?: PostSelect
    
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
     where?: PostWhereInput
    
}


/**
 * **Post find first args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostFindFirstArgs = {
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: PostWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: PostSerializableScalarFields
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: PostInclude
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<PostOrderByInput>
    
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
     select?: PostSelect
    
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
     where?: PostWhereInput
    
}


/**
 * **Post find unique args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostFindUniqueArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: PostInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: PostSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereUniqueInput
    
}


/**
 * **Post create args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostCreateArgs = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: PostCreateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: PostInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: PostSelect
    
}


/**
 * **Post update args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostUpdateArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: PostInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: PostSelect
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: PostUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereUniqueInput
    
}


/**
 * **Post upsert args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostUpsertArgs = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: PostCreateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: PostInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: PostSelect
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: PostUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereUniqueInput
    
}


/**
 * **Post copy args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostCopyArgs = {
    
    /**
     * **Copy**
     *
     * This synthesized field doesn't have a description.
     */
     copy: PostUpdateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: PostInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: PostSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereUniqueInput
    
}


/**
 * **Post delete args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostDeleteArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: PostInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: PostSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereUniqueInput
    
}


/**
 * **Post create many args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostCreateManyArgs = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: Enumerable<PostCreateInput>
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: PostInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: PostSelect
    
}


/**
 * **Post update many args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostUpdateManyArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: PostInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: PostSelect
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: PostUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereInput
    
}


/**
 * **Post delete many args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostDeleteManyArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: PostInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: PostSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereInput
    
}


/**
 * **Post copy many args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostCopyManyArgs = {
    
    /**
     * **Copy**
     *
     * This synthesized field doesn't have a description.
     */
     copy: PostUpdateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: PostInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: PostSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: PostWhereInput
    
}


/**
 * **Post count args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostCountArgs = {
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: PostWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: PostSerializableScalarFields
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<PostOrderByInput>
    
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
     select?: PostCountAggregateInputType
    
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
     where?: PostWhereInput
    
}


/**
 * **Post aggregate args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostAggregateArgs = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: PostAvgAggregateInputType
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: PostCountAggregateInputType
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: PostMaxAggregateInputType
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: PostMinAggregateInputType
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: PostSumAggregateInputType
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: PostWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: PostSerializableScalarFields
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<PostOrderByInput>
    
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
     where?: PostWhereInput
    
}


/**
 * **Post group by args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostGroupByArgs = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: PostAvgAggregateInputType
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: PostCountAggregateInputType
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: PostMaxAggregateInputType
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: PostMinAggregateInputType
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: PostSumAggregateInputType
    
    /**
     * **By**
     *
     * This synthesized field doesn't have a description.
     */
     by: Enumerable<PostSerializableScalarFields>
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: PostWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: PostSerializableScalarFields
    
    /**
     * **Having**
     *
     * This synthesized field doesn't have a description.
     */
     having?: PostScalarWhereWithAggregatesInput
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<PostOrderByInput>
    
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
     where?: PostWhereInput
    
}


/**
 * **Post scalar update input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostScalarUpdateInput = {
    
    /**
     * **Author Id**
     *
     * This synthesized field doesn't have a description.
     */
     authorId?: number
    
    /**
     * **Content**
     *
     * This synthesized field doesn't have a description.
     */
     content?: string
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Published**
     *
     * This synthesized field doesn't have a description.
     */
     published?: boolean
    
    /**
     * **Title**
     *
     * This synthesized field doesn't have a description.
     */
     title?: string
    
}


/**
 * **Post sign in checker ids**
 *
 * This synthesized interface doesn't have a description
 */
export type PostSignInCheckerIds = {
    
}


/**
 * **Post sign in checker companions**
 *
 * This synthesized interface doesn't have a description
 */
export type PostSignInCheckerCompanions = {
    
}


/**
 * **Post sign in input**
 *
 * This synthesized interface doesn't have a description
 */
export type PostSignInInput = {
    
    /**
     * **Credentials**
     *
     * This synthesized field doesn't have a description.
     */
     credentials: PostSignInArgs
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: PostInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: PostSelect
    
}


/**
 * **Post sign in args**
 *
 * This synthesized interface doesn't have a description
 */
export type PostSignInArgs = {
    
}


/**
 * **User select**
 *
 * This synthesized interface doesn't have a description
 */
export type UserSelect = {
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: boolean
    
}


/**
 * **User include**
 *
 * This synthesized interface doesn't have a description
 */
export type UserInclude = {
    
    /**
     * **Posts**
     *
     * This synthesized field doesn't have a description.
     */
     posts?: PostFindManyArgs | boolean
    
}


/**
 * **User where input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserWhereInput = {
    
    /**
     * **And**
     *
     * This synthesized field doesn't have a description.
     */
     AND?: UserWhereInput[]
    
    /**
     * **Not**
     *
     * This synthesized field doesn't have a description.
     */
     NOT?: UserWhereInput
    
    /**
     * **Or**
     *
     * This synthesized field doesn't have a description.
     */
     OR?: UserWhereInput[]
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email?: string | std.StringFilter
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number | std.Filter<number>
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: string | null | std.StringNullableFilter
    
    /**
     * **Posts**
     *
     * This synthesized field doesn't have a description.
     */
     posts?: PostListRelationFilter
    
}


/**
 * **User where unique input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserWhereUniqueInput = {
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email?: string
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
}


/**
 * **User scalar where with aggregates input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserScalarWhereWithAggregatesInput = {
    
    /**
     * **And**
     *
     * This synthesized field doesn't have a description.
     */
     AND?: UserWhereInput[]
    
    /**
     * **Not**
     *
     * This synthesized field doesn't have a description.
     */
     NOT?: UserWhereInput
    
    /**
     * **Or**
     *
     * This synthesized field doesn't have a description.
     */
     OR?: UserWhereInput[]
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email?: string | std.StringWithAggregatesFilter
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number | std.IntNumberWithAggregatesFilter<number>
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: string | null | std.StringNullableWithAggregatesFilter
    
}


/**
 * **User relation filter**
 *
 * This synthesized interface doesn't have a description
 */
export type UserRelationFilter = {
    
    /**
     * **Is**
     *
     * This synthesized field doesn't have a description.
     */
     is?: UserWhereInput
    
    /**
     * **Is Not**
     *
     * This synthesized field doesn't have a description.
     */
     isNot?: UserWhereInput
    
}


/**
 * **User list relation filter**
 *
 * This synthesized interface doesn't have a description
 */
export type UserListRelationFilter = {
    
    /**
     * **Every**
     *
     * This synthesized field doesn't have a description.
     */
     every?: UserWhereInput
    
    /**
     * **None**
     *
     * This synthesized field doesn't have a description.
     */
     none?: UserWhereInput
    
    /**
     * **Some**
     *
     * This synthesized field doesn't have a description.
     */
     some?: UserWhereInput
    
}


/**
 * **User order by input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserOrderByInput = {
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email?: std.Sort
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: std.Sort
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: std.Sort
    
}


/**
 * **User count aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type UserCountAggregateInputType = {
    
    /**
     * **All**
     *
     * This synthesized field doesn't have a description.
     */
     _all?: boolean
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: boolean
    
}


/**
 * **User sum aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type UserSumAggregateInputType = {
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
}


/**
 * **User avg aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type UserAvgAggregateInputType = {
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
}


/**
 * **User min aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type UserMinAggregateInputType = {
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: boolean
    
}


/**
 * **User max aggregate input type**
 *
 * This synthesized interface doesn't have a description
 */
export type UserMaxAggregateInputType = {
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email?: boolean
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: boolean
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: boolean
    
}


/**
 * **User create input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserCreateInput = {
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email: string
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: string
    
    /**
     * **Posts**
     *
     * This synthesized field doesn't have a description.
     */
     posts?: PostCreateNestedManyWithoutAuthorInput
    
}


/**
 * **User create without posts input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserCreateWithoutPostsInput = {
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email: string
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: string
    
}


/**
 * **User update input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserUpdateInput = {
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email?: string
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: string
    
    /**
     * **Posts**
     *
     * This synthesized field doesn't have a description.
     */
     posts?: PostUpdateNestedManyWithoutAuthorInput
    
}


/**
 * **User update without posts input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserUpdateWithoutPostsInput = {
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email?: string
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: string
    
}


/**
 * **User create nested one input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserCreateNestedOneInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: UserWhereUniqueInput
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: UserConnectOrCreateInput
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: UserCreateInput
    
}


/**
 * **User create nested one without posts input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserCreateNestedOneWithoutPostsInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: UserWhereUniqueInput
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: UserConnectOrCreateWithoutPostsInput
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: UserCreateInput
    
}


/**
 * **User create nested many input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserCreateNestedManyInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: Enumerable<UserWhereUniqueInput>
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: Enumerable<UserConnectOrCreateInput>
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: Enumerable<UserCreateInput>
    
}


/**
 * **User create nested many without posts input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserCreateNestedManyWithoutPostsInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: Enumerable<UserWhereUniqueInput>
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: Enumerable<UserConnectOrCreateWithoutPostsInput>
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: Enumerable<UserCreateInput>
    
}


/**
 * **User update nested one input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserUpdateNestedOneInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: UserWhereUniqueInput
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: UserConnectOrCreateInput
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: UserCreateInput
    
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
     set?: UserWhereUniqueInput
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update?: UserUpdateInput
    
    /**
     * **Upsert**
     *
     * This synthesized field doesn't have a description.
     */
     upsert?: UserUpsertWithWhereUniqueInput
    
}


/**
 * **User update nested one without posts input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserUpdateNestedOneWithoutPostsInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: UserWhereUniqueInput
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: UserConnectOrCreateWithoutPostsInput
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: UserCreateInput
    
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
     set?: UserWhereUniqueInput
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update?: UserUpdateInput
    
    /**
     * **Upsert**
     *
     * This synthesized field doesn't have a description.
     */
     upsert?: UserUpsertWithWhereUniqueWithoutPostsInput
    
}


/**
 * **User update nested many input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserUpdateNestedManyInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: Enumerable<UserWhereUniqueInput>
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: Enumerable<UserConnectOrCreateInput>
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: Enumerable<UserCreateInput>
    
    /**
     * **Delete**
     *
     * This synthesized field doesn't have a description.
     */
     delete?: Enumerable<UserWhereUniqueInput>
    
    /**
     * **Delete Many**
     *
     * This synthesized field doesn't have a description.
     */
     deleteMany?: Enumerable<UserWhereInput>
    
    /**
     * **Disconnect**
     *
     * This synthesized field doesn't have a description.
     */
     disconnect?: Enumerable<UserWhereUniqueInput>
    
    /**
     * **Set**
     *
     * This synthesized field doesn't have a description.
     */
     set?: Enumerable<UserWhereUniqueInput>
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update?: Enumerable<UserUpdateWithWhereUniqueInput>
    
    /**
     * **Update Many**
     *
     * This synthesized field doesn't have a description.
     */
     updateMany?: Enumerable<UserUpdateManyWithWhereInput>
    
    /**
     * **Upsert**
     *
     * This synthesized field doesn't have a description.
     */
     upsert?: Enumerable<UserUpsertWithWhereUniqueInput>
    
}


/**
 * **User update nested many without posts input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserUpdateNestedManyWithoutPostsInput = {
    
    /**
     * **Connect**
     *
     * This synthesized field doesn't have a description.
     */
     connect?: Enumerable<UserWhereUniqueInput>
    
    /**
     * **Connect Or Create**
     *
     * This synthesized field doesn't have a description.
     */
     connectOrCreate?: Enumerable<UserConnectOrCreateWithoutPostsInput>
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create?: Enumerable<UserCreateInput>
    
    /**
     * **Delete**
     *
     * This synthesized field doesn't have a description.
     */
     delete?: Enumerable<UserWhereUniqueInput>
    
    /**
     * **Delete Many**
     *
     * This synthesized field doesn't have a description.
     */
     deleteMany?: Enumerable<UserWhereInput>
    
    /**
     * **Disconnect**
     *
     * This synthesized field doesn't have a description.
     */
     disconnect?: Enumerable<UserWhereUniqueInput>
    
    /**
     * **Set**
     *
     * This synthesized field doesn't have a description.
     */
     set?: Enumerable<UserWhereUniqueInput>
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update?: Enumerable<UserUpdateWithWhereUniqueWithoutPostsInput>
    
    /**
     * **Update Many**
     *
     * This synthesized field doesn't have a description.
     */
     updateMany?: Enumerable<UserUpdateManyWithWhereWithoutPostsInput>
    
    /**
     * **Upsert**
     *
     * This synthesized field doesn't have a description.
     */
     upsert?: Enumerable<UserUpsertWithWhereUniqueWithoutPostsInput>
    
}


/**
 * **User connect or create input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserConnectOrCreateInput = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: UserCreateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereUniqueInput
    
}


/**
 * **User connect or create without posts input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserConnectOrCreateWithoutPostsInput = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: UserCreateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereUniqueInput
    
}


/**
 * **User update with where unique input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserUpdateWithWhereUniqueInput = {
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: UserUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereUniqueInput
    
}


/**
 * **User update with where unique without posts input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserUpdateWithWhereUniqueWithoutPostsInput = {
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: UserUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereUniqueInput
    
}


/**
 * **User upsert with where unique input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserUpsertWithWhereUniqueInput = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: UserCreateInput
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: UserUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereUniqueInput
    
}


/**
 * **User upsert with where unique without posts input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserUpsertWithWhereUniqueWithoutPostsInput = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: UserCreateInput
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: UserUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereUniqueInput
    
}


/**
 * **User update many with where input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserUpdateManyWithWhereInput = {
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: UserUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereInput
    
}


/**
 * **User update many with where without posts input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserUpdateManyWithWhereWithoutPostsInput = {
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: UserUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereInput
    
}


/**
 * **User**
 *
 * This synthesized interface doesn't have a description
 */
export type User = {
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email: string
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id: number
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: string
    
    /**
     * **Posts**
     *
     * This synthesized field doesn't have a description.
     */
     posts: Post[]
    
}
export type UserGetPayload<S extends boolean | null | undefined | UserArgs, U = keyof S> = S extends true
    ? User
    : S extends undefined
        ? never
        : S extends UserArgs | UserFindManyArgs
            ? 'include' extends U
                ? SelectSubset<User, S> & {
                    [P in ExistKeys<S['include']>]:
                        P extends 'posts' ? PostGetPayload<S['include'][P]>[] :
                    never
                }
                : SelectSubset<User, S>
            : User

export type GetUserAggregateType<T extends UserAggregateArgs> = {
    [P in keyof T & keyof UserAggregateResult]: P extends '_count' | 'count'
  ? T[P] extends true
    ? number
    : GetScalarType<T[P], UserAggregateResult[P]>
  : GetScalarType<T[P], UserAggregateResult[P]>
}

export type GetUserGroupByPayload<T extends UserGroupByArgs> =
  Array<
    PickEnumerable<UserGroupByResult, T['by']> &
      {
        [P in ((keyof T) & (keyof UserGroupByResult))]: P extends '_count'
          ? T[P] extends boolean
            ? number
            : GetScalarType<T[P], UserGroupByResult[P]>
          : GetScalarType<T[P], UserGroupByResult[P]>
      }
    >


/**
 * **User count aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type UserCountAggregateResult = {
    
    /**
     * **All**
     *
     * This synthesized field doesn't have a description.
     */
     _all?: number
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email?: number
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: number
    
}


/**
 * **User sum aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type UserSumAggregateResult = {
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
}


/**
 * **User avg aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type UserAvgAggregateResult = {
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
}


/**
 * **User min aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type UserMinAggregateResult = {
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email?: string
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: string
    
}


/**
 * **User max aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type UserMaxAggregateResult = {
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email?: string
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: string
    
}


/**
 * **User aggregate result**
 *
 * This synthesized interface doesn't have a description
 */
export type UserAggregateResult = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: UserAvgAggregateResult
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: UserCountAggregateResult
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: UserMaxAggregateResult
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: UserMinAggregateResult
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: UserSumAggregateResult
    
}


/**
 * **User group by result**
 *
 * This synthesized interface doesn't have a description
 */
export type UserGroupByResult = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: UserAvgAggregateResult
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: UserCountAggregateResult
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: UserMaxAggregateResult
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: UserMinAggregateResult
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: UserSumAggregateResult
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email?: string
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: string
    
}


/**
 * **User args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: UserInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: UserSelect
    
}


/**
 * **User find many args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserFindManyArgs = {
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: UserWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: UserSerializableScalarFields
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: UserInclude
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<UserOrderByInput>
    
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
     select?: UserSelect
    
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
     where?: UserWhereInput
    
}


/**
 * **User find first args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserFindFirstArgs = {
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: UserWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: UserSerializableScalarFields
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: UserInclude
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<UserOrderByInput>
    
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
     select?: UserSelect
    
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
     where?: UserWhereInput
    
}


/**
 * **User find unique args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserFindUniqueArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: UserInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: UserSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereUniqueInput
    
}


/**
 * **User create args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserCreateArgs = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: UserCreateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: UserInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: UserSelect
    
}


/**
 * **User update args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserUpdateArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: UserInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: UserSelect
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: UserUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereUniqueInput
    
}


/**
 * **User upsert args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserUpsertArgs = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: UserCreateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: UserInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: UserSelect
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: UserUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereUniqueInput
    
}


/**
 * **User copy args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserCopyArgs = {
    
    /**
     * **Copy**
     *
     * This synthesized field doesn't have a description.
     */
     copy: UserUpdateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: UserInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: UserSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereUniqueInput
    
}


/**
 * **User delete args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserDeleteArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: UserInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: UserSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereUniqueInput
    
}


/**
 * **User create many args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserCreateManyArgs = {
    
    /**
     * **Create**
     *
     * This synthesized field doesn't have a description.
     */
     create: Enumerable<UserCreateInput>
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: UserInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: UserSelect
    
}


/**
 * **User update many args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserUpdateManyArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: UserInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: UserSelect
    
    /**
     * **Update**
     *
     * This synthesized field doesn't have a description.
     */
     update: UserUpdateInput
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereInput
    
}


/**
 * **User delete many args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserDeleteManyArgs = {
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: UserInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: UserSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereInput
    
}


/**
 * **User copy many args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserCopyManyArgs = {
    
    /**
     * **Copy**
     *
     * This synthesized field doesn't have a description.
     */
     copy: UserUpdateInput
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: UserInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: UserSelect
    
    /**
     * **Where**
     *
     * This synthesized field doesn't have a description.
     */
     where: UserWhereInput
    
}


/**
 * **User count args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserCountArgs = {
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: UserWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: UserSerializableScalarFields
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<UserOrderByInput>
    
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
     select?: UserCountAggregateInputType
    
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
     where?: UserWhereInput
    
}


/**
 * **User aggregate args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserAggregateArgs = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: UserAvgAggregateInputType
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: UserCountAggregateInputType
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: UserMaxAggregateInputType
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: UserMinAggregateInputType
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: UserSumAggregateInputType
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: UserWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: UserSerializableScalarFields
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<UserOrderByInput>
    
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
     where?: UserWhereInput
    
}


/**
 * **User group by args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserGroupByArgs = {
    
    /**
     * **Avg**
     *
     * This synthesized field doesn't have a description.
     */
     _avg?: UserAvgAggregateInputType
    
    /**
     * **Count**
     *
     * This synthesized field doesn't have a description.
     */
     _count?: UserCountAggregateInputType
    
    /**
     * **Max**
     *
     * This synthesized field doesn't have a description.
     */
     _max?: UserMaxAggregateInputType
    
    /**
     * **Min**
     *
     * This synthesized field doesn't have a description.
     */
     _min?: UserMinAggregateInputType
    
    /**
     * **Sum**
     *
     * This synthesized field doesn't have a description.
     */
     _sum?: UserSumAggregateInputType
    
    /**
     * **By**
     *
     * This synthesized field doesn't have a description.
     */
     by: Enumerable<UserSerializableScalarFields>
    
    /**
     * **Cursor**
     *
     * This synthesized field doesn't have a description.
     */
     cursor?: UserWhereUniqueInput
    
    /**
     * **Distinct**
     *
     * This synthesized field doesn't have a description.
     */
     distinct?: UserSerializableScalarFields
    
    /**
     * **Having**
     *
     * This synthesized field doesn't have a description.
     */
     having?: UserScalarWhereWithAggregatesInput
    
    /**
     * **Order By**
     *
     * This synthesized field doesn't have a description.
     */
     orderBy?: Enumerable<UserOrderByInput>
    
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
     where?: UserWhereInput
    
}


/**
 * **User scalar update input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserScalarUpdateInput = {
    
    /**
     * **Email**
     *
     * This synthesized field doesn't have a description.
     */
     email?: string
    
    /**
     * **Id**
     *
     * This synthesized field doesn't have a description.
     */
     id?: number
    
    /**
     * **Name**
     *
     * This synthesized field doesn't have a description.
     */
     name?: string
    
}


/**
 * **User sign in checker ids**
 *
 * This synthesized interface doesn't have a description
 */
export type UserSignInCheckerIds = {
    
}


/**
 * **User sign in checker companions**
 *
 * This synthesized interface doesn't have a description
 */
export type UserSignInCheckerCompanions = {
    
}


/**
 * **User sign in input**
 *
 * This synthesized interface doesn't have a description
 */
export type UserSignInInput = {
    
    /**
     * **Credentials**
     *
     * This synthesized field doesn't have a description.
     */
     credentials: UserSignInArgs
    
    /**
     * **Include**
     *
     * This synthesized field doesn't have a description.
     */
     include?: UserInclude
    
    /**
     * **Select**
     *
     * This synthesized field doesn't have a description.
     */
     select?: UserSelect
    
}


/**
 * **User sign in args**
 *
 * This synthesized interface doesn't have a description
 */
export type UserSignInArgs = {
    
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







        export interface AdminNamespaceDelegate {

            

            

            

            /**
             * Get a new client altered with `headers`.
             * @param {headers?} headers - The new headers.
             */
            $headers(headers?: {[key: string]: string} | undefined): Teo
        }





    }

    export namespace bcrypt {







        export interface BcryptNamespaceDelegate {

            

            

            

            /**
             * Get a new client altered with `headers`.
             * @param {headers?} headers - The new headers.
             */
            $headers(headers?: {[key: string]: string} | undefined): Teo
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






        export interface IdentityNamespaceDelegate {

            
            identity<T extends FFUCKArgsFUCKK>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<FFUCKResultFUCKK>>
            
            signIn<T extends F2FUCKstd.identity.SignInInputFUCK2K>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.DataMeta<FFUCKResultFUCKK, std.identity.TokenInfo>>
            

            

            

            /**
             * Get a new client altered with `headers`.
             * @param {headers?} headers - The new headers.
             */
            $headers(headers?: {[key: string]: string} | undefined): Teo
        }





    }




    export interface StdNamespaceDelegate {

        

        
        admin: std.admin.AdminNamespaceDelegate
        
        bcrypt: std.bcrypt.BcryptNamespaceDelegate
        
        identity: std.identity.IdentityNamespaceDelegate
        

        

        /**
         * Get a new client altered with `headers`.
         * @param {headers?} headers - The new headers.
         */
        $headers(headers?: {[key: string]: string} | undefined): Teo
    }





}




export interface PostDelegate {

    
    findUnique<T extends PostFindUniqueArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<CheckSelectInclude<T, Post, PostGetPayload<T>>>>
    
    findFirst<T extends PostFindFirstArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<CheckSelectInclude<T, Post, PostGetPayload<T>>>>
    
    findMany<T extends PostFindManyArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.DataMeta<CheckSelectInclude<T, Post, PostGetPayload<T>>[], std.PagingInfo>>
    
    create<T extends PostCreateArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<CheckSelectInclude<T, Post, PostGetPayload<T>>>>
    
    update<T extends PostUpdateArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<CheckSelectInclude<T, Post, PostGetPayload<T>>>>
    
    upsert<T extends PostUpsertArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<CheckSelectInclude<T, Post, PostGetPayload<T>>>>
    
    copy<T extends PostCopyArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<CheckSelectInclude<T, Post, PostGetPayload<T>>>>
    
    delete<T extends PostDeleteArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<CheckSelectInclude<T, Post, PostGetPayload<T>>>>
    
    createMany<T extends PostCreateManyArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.DataMeta<CheckSelectInclude<T, Post, PostGetPayload<T>>[], std.PagingInfo>>
    
    updateMany<T extends PostUpdateManyArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.DataMeta<CheckSelectInclude<T, Post, PostGetPayload<T>>[], std.PagingInfo>>
    
    copyMany<T extends PostCopyManyArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.DataMeta<CheckSelectInclude<T, Post, PostGetPayload<T>>[], std.PagingInfo>>
    
    deleteMany<T extends PostDeleteManyArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.DataMeta<CheckSelectInclude<T, Post, PostGetPayload<T>>[], std.PagingInfo>>
    
    count<T extends PostCountArgs>(body: Subset<T, PostCountArgs>,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<T extends Record<'select', any>
       ? T['select'] extends true
         ? number
         : GetScalarType<T['select'], PostCountAggregateResult>
       : number>>
    
    aggregate<T extends PostAggregateArgs>(body: Subset<T, PostAggregateArgs>,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<GetPostAggregateType<T>>>
    
    groupBy<T extends PostGroupByArgs,
      HasSelectOrTake extends Or<
        Extends<'skip', Keys<T>>,
        Extends<'take', Keys<T>>
      >,
      OrderByArg extends True extends HasSelectOrTake
        ? { orderBy: PostGroupByArgs['orderBy'] }
        : { orderBy?: PostGroupByArgs['orderBy'] },
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
        }[OrderFields]>(body: SubsetIntersection<T, PostGroupByArgs, OrderByArg> & InputErrors,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<{} extends InputErrors ? GetPostGroupByPayload<T> : InputErrors>>
    

    

    

    /**
     * Get a new client altered with `headers`.
     * @param {headers?} headers - The new headers.
     */
    $headers(headers?: {[key: string]: string} | undefined): Teo
}

export interface UserDelegate {

    
    findUnique<T extends UserFindUniqueArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<CheckSelectInclude<T, User, UserGetPayload<T>>>>
    
    findFirst<T extends UserFindFirstArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<CheckSelectInclude<T, User, UserGetPayload<T>>>>
    
    findMany<T extends UserFindManyArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.DataMeta<CheckSelectInclude<T, User, UserGetPayload<T>>[], std.PagingInfo>>
    
    create<T extends UserCreateArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<CheckSelectInclude<T, User, UserGetPayload<T>>>>
    
    update<T extends UserUpdateArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<CheckSelectInclude<T, User, UserGetPayload<T>>>>
    
    upsert<T extends UserUpsertArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<CheckSelectInclude<T, User, UserGetPayload<T>>>>
    
    copy<T extends UserCopyArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<CheckSelectInclude<T, User, UserGetPayload<T>>>>
    
    delete<T extends UserDeleteArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<CheckSelectInclude<T, User, UserGetPayload<T>>>>
    
    createMany<T extends UserCreateManyArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.DataMeta<CheckSelectInclude<T, User, UserGetPayload<T>>[], std.PagingInfo>>
    
    updateMany<T extends UserUpdateManyArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.DataMeta<CheckSelectInclude<T, User, UserGetPayload<T>>[], std.PagingInfo>>
    
    copyMany<T extends UserCopyManyArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.DataMeta<CheckSelectInclude<T, User, UserGetPayload<T>>[], std.PagingInfo>>
    
    deleteMany<T extends UserDeleteManyArgs>(body: T,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.DataMeta<CheckSelectInclude<T, User, UserGetPayload<T>>[], std.PagingInfo>>
    
    count<T extends UserCountArgs>(body: Subset<T, UserCountArgs>,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<T extends Record<'select', any>
       ? T['select'] extends true
         ? number
         : GetScalarType<T['select'], UserCountAggregateResult>
       : number>>
    
    aggregate<T extends UserAggregateArgs>(body: Subset<T, UserAggregateArgs>,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<GetUserAggregateType<T>>>
    
    groupBy<T extends UserGroupByArgs,
      HasSelectOrTake extends Or<
        Extends<'skip', Keys<T>>,
        Extends<'take', Keys<T>>
      >,
      OrderByArg extends True extends HasSelectOrTake
        ? { orderBy: UserGroupByArgs['orderBy'] }
        : { orderBy?: UserGroupByArgs['orderBy'] },
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
        }[OrderFields]>(body: SubsetIntersection<T, UserGroupByArgs, OrderByArg> & InputErrors,headers?: {[key: string]: string} | undefined, queryString?: string | undefined): Promise<std.Data<{} extends InputErrors ? GetUserGroupByPayload<T> : InputErrors>>
    

    

    

    /**
     * Get a new client altered with `headers`.
     * @param {headers?} headers - The new headers.
     */
    $headers(headers?: {[key: string]: string} | undefined): Teo
}

export interface Teo {

    

    
    std: std.StdNamespaceDelegate
    

    
    post: PostDelegate
    
    user: UserDelegate
    

    /**
     * Get a new client altered with `headers`.
     * @param {headers?} headers - The new headers.
     */
    $headers(headers?: {[key: string]: string} | undefined): Teo
}



/**
 * ## Teo API Client
 *
 * Teo API client for TypeScript & javaScript. It supports both browser and
 * node.js. It's generated by the fantastic Teo framework.
 *
 */
export const teo: Teo


