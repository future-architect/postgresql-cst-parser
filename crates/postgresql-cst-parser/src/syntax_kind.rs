use cstree::Syntax;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Syntax)]
#[repr(u32)]
pub enum SyntaxKind {
    MODE_TYPE_NAME,
    MODE_PLPGSQL_EXPR,
    MODE_PLPGSQL_ASSIGN1,
    MODE_PLPGSQL_ASSIGN2,
    MODE_PLPGSQL_ASSIGN3,
    Semicolon,
    CONCURRENTLY,
    CASCADE,
    RESTRICT,
    CALL,
    CREATE,
    ROLE,
    WITH,
    WITH_LA,
    PASSWORD,
    PARAM,
    NULL_P,
    ENCRYPTED,
    UNENCRYPTED,
    INHERIT,
    CONNECTION,
    LIMIT,
    VALID,
    UNTIL,
    USER,
    IDENT,
    SYSID,
    ADMIN,
    IN_P,
    GROUP_P,
    ALTER,
    DATABASE,
    ALL,
    DROP,
    IF_P,
    EXISTS,
    ADD_P,
    SCHEMA,
    AUTHORIZATION,
    NOT,
    SET,
    LOCAL,
    SESSION,
    TRANSACTION,
    CHARACTERISTICS,
    AS,
    TO,
    Equals,
    DEFAULT,
    FROM,
    CURRENT_P,
    TIME,
    ZONE,
    CATALOG_P,
    NAMES,
    XML_P,
    OPTION,
    SNAPSHOT,
    Dot,
    Comma,
    READ,
    UNCOMMITTED,
    COMMITTED,
    REPEATABLE,
    SERIALIZABLE,
    TRUE_P,
    FALSE_P,
    ON,
    LParen,
    RParen,
    RESET,
    ISOLATION,
    LEVEL,
    SHOW,
    CONSTRAINTS,
    DEFERRED,
    IMMEDIATE,
    CHECKPOINT,
    DISCARD,
    TEMP,
    TEMPORARY,
    PLANS,
    SEQUENCES,
    TABLE,
    TABLESPACE,
    OWNED,
    BY,
    INDEX,
    SEQUENCE,
    VIEW,
    MATERIALIZED,
    FOREIGN,
    ATTACH,
    PARTITION,
    DETACH,
    FINALIZE,
    COLUMN,
    EXPRESSION,
    STATISTICS,
    GENERATED,
    IDENTITY_P,
    TYPE_P,
    CONSTRAINT,
    VALIDATE,
    WITHOUT,
    OIDS,
    CLUSTER,
    LOGGED,
    UNLOGGED,
    ENABLE_P,
    TRIGGER,
    ALWAYS,
    REPLICA,
    DISABLE_P,
    RULE,
    NO,
    OF,
    OWNER,
    ACCESS,
    METHOD,
    ROW,
    SECURITY,
    FORCE,
    COLLATE,
    USING,
    NOTHING,
    FULL,
    RESTART,
    FOR,
    VALUES,
    ATTRIBUTE,
    CLOSE,
    COPY,
    PROGRAM,
    STDIN,
    STDOUT,
    BINARY,
    FREEZE,
    DELIMITER,
    CSV,
    HEADER_P,
    QUOTE,
    ESCAPE,
    Star,
    ENCODING,
    DELIMITERS,
    GLOBAL,
    OPTIONS,
    COMPRESSION,
    STORAGE,
    UNIQUE,
    PRIMARY,
    KEY,
    CHECK,
    STORED,
    REFERENCES,
    NULLS_P,
    DISTINCT,
    DEFERRABLE,
    INITIALLY,
    LIKE,
    INCLUDING,
    EXCLUDING,
    COMMENTS,
    DEFAULTS,
    INDEXES,
    EXCLUDE,
    INCLUDE,
    MATCH,
    PARTIAL,
    SIMPLE,
    OPERATOR,
    WHERE,
    UPDATE,
    DELETE_P,
    ACTION,
    INHERITS,
    COMMIT,
    ROWS,
    PRESERVE,
    DATA_P,
    REFRESH,
    CACHE,
    CYCLE,
    INCREMENT,
    MAXVALUE,
    MINVALUE,
    NAME_P,
    START,
    FCONST,
    Plus,
    Minus,
    LANGUAGE,
    HANDLER,
    TRUSTED,
    INLINE_P,
    VALIDATOR,
    PROCEDURAL,
    LOCATION,
    EXTENSION,
    VERSION_P,
    AGGREGATE,
    CAST,
    DOMAIN_P,
    FUNCTION,
    CLASS,
    FAMILY,
    PROCEDURE,
    ROUTINE,
    TRANSFORM,
    WRAPPER,
    SERVER,
    IMPORT_P,
    INTO,
    EXCEPT,
    MAPPING,
    POLICY,
    SELECT,
    INSERT,
    EXECUTE,
    AFTER,
    EACH,
    BEFORE,
    INSTEAD,
    OR,
    TRUNCATE,
    REFERENCING,
    NEW,
    OLD,
    STATEMENT,
    WHEN,
    EVENT,
    AND,
    SCONST,
    ASSERTION,
    ENUM_P,
    RANGE,
    TEXT_P,
    SEARCH,
    PARSER,
    DICTIONARY,
    TEMPLATE,
    CONFIGURATION,
    COLLATION,
    NONE,
    VALUE_P,
    RENAME,
    ORDER,
    RECHECK,
    REASSIGN,
    CONVERSION_P,
    SUBSCRIPTION,
    PUBLICATION,
    CONTINUE_P,
    COMMENT,
    IS,
    LARGE_P,
    OBJECT_P,
    LABEL,
    FETCH,
    MOVE,
    NEXT,
    PRIOR,
    FIRST_P,
    LAST_P,
    ABSOLUTE_P,
    RELATIVE_P,
    FORWARD,
    BACKWARD,
    GRANT,
    REVOKE,
    PRIVILEGES,
    SYSTEM_P,
    PARAMETER,
    TABLES,
    FUNCTIONS,
    PROCEDURES,
    ROUTINES,
    GRANTED,
    TYPES_P,
    SCHEMAS,
    ASC,
    DESC,
    NULLS_LA,
    RETURNS,
    REPLACE,
    OUT_P,
    INOUT,
    VARIADIC,
    Percent,
    SETOF,
    CALLED,
    INPUT_P,
    STRICT_P,
    IMMUTABLE,
    STABLE,
    VOLATILE,
    EXTERNAL,
    DEFINER,
    INVOKER,
    LEAKPROOF,
    COST,
    SUPPORT,
    PARALLEL,
    WINDOW,
    RETURN,
    BEGIN_P,
    ATOMIC,
    END_P,
    DO,
    IMPLICIT_P,
    ASSIGNMENT,
    SQL_P,
    REINDEX,
    DEPENDS,
    CURRENT_SCHEMA,
    SKIP,
    ALSO,
    NOTIFY,
    LISTEN,
    UNLISTEN,
    ABORT_P,
    ROLLBACK,
    SAVEPOINT,
    RELEASE,
    PREPARE,
    PREPARED,
    WORK,
    ONLY,
    WRITE,
    CHAIN,
    RECURSIVE,
    CASCADED,
    LOAD,
    VACUUM,
    ANALYZE,
    ANALYSE,
    FORMAT_LA,
    VERBOSE,
    EXPLAIN,
    DEALLOCATE,
    OVERRIDING,
    CONFLICT,
    RETURNING,
    LOCK_P,
    MODE,
    SHARE,
    EXCLUSIVE,
    NOWAIT,
    LOCKED,
    MERGE,
    MATCHED,
    THEN,
    DECLARE,
    CURSOR,
    SCROLL,
    ASENSITIVE,
    INSENSITIVE,
    HOLD,
    UNION,
    INTERSECT,
    DEPTH,
    BREADTH,
    TIES,
    OFFSET,
    ROLLUP,
    CUBE,
    GROUPING,
    SETS,
    HAVING,
    LATERAL_P,
    CROSS,
    JOIN,
    NATURAL,
    LEFT,
    RIGHT,
    INNER_P,
    OUTER_P,
    TABLESAMPLE,
    ORDINALITY,
    XMLTABLE,
    COLUMNS,
    XMLNAMESPACES,
    ARRAY,
    LBracket,
    RBracket,
    INT_P,
    INTEGER,
    SMALLINT,
    BIGINT,
    REAL,
    FLOAT_P,
    DOUBLE_P,
    PRECISION,
    DECIMAL_P,
    DEC,
    NUMERIC,
    BOOLEAN_P,
    BIT,
    CHARACTER,
    CHAR_P,
    VARCHAR,
    NATIONAL,
    NCHAR,
    VARYING,
    TIMESTAMP,
    INTERVAL,
    WITHOUT_LA,
    YEAR_P,
    MONTH_P,
    DAY_P,
    HOUR_P,
    MINUTE_P,
    SECOND_P,
    TYPECAST,
    AT,
    Slash,
    Caret,
    Less,
    Greater,
    LESS_EQUALS,
    GREATER_EQUALS,
    NOT_EQUALS,
    NOT_LA,
    ILIKE,
    SIMILAR,
    ISNULL,
    NOTNULL,
    OVERLAPS,
    UNKNOWN,
    BETWEEN,
    SYMMETRIC,
    DOCUMENT_P,
    NORMALIZED,
    CURRENT_DATE,
    CURRENT_TIME,
    CURRENT_TIMESTAMP,
    LOCALTIME,
    LOCALTIMESTAMP,
    CURRENT_ROLE,
    CURRENT_USER,
    SESSION_USER,
    SYSTEM_USER,
    CURRENT_CATALOG,
    EXTRACT,
    NORMALIZE,
    OVERLAY,
    POSITION,
    SUBSTRING,
    TREAT,
    TRIM,
    BOTH,
    LEADING,
    TRAILING,
    NULLIF,
    COALESCE,
    GREATEST,
    LEAST,
    XMLCONCAT,
    XMLELEMENT,
    XMLEXISTS,
    XMLFOREST,
    XMLPARSE,
    XMLPI,
    XMLROOT,
    XMLSERIALIZE,
    JSON_OBJECT,
    JSON_ARRAY,
    STANDALONE_P,
    YES_P,
    XMLATTRIBUTES,
    CONTENT_P,
    INDENT,
    WHITESPACE_P,
    STRIP_P,
    PASSING,
    REF_P,
    WITHIN,
    FILTER,
    OVER,
    GROUPS,
    UNBOUNDED,
    PRECEDING,
    FOLLOWING,
    OTHERS,
    ANY,
    SOME,
    Op,
    COLON_EQUALS,
    EQUALS_GREATER,
    NFC,
    NFD,
    NFKC,
    NFKD,
    PLACING,
    CASE,
    ELSE,
    Colon,
    ASYMMETRIC,
    JSON,
    SCALAR,
    KEYS,
    ABSENT,
    JSON_OBJECTAGG,
    JSON_ARRAYAGG,
    BCONST,
    XCONST,
    ICONST,
    FORMAT,
    OFF,
    UESCAPE,
    VIEWS,
    Dollarend,
    stmtmulti,
    Typename,
    PLpgSQL_Expr,
    PLAssignStmt,
    toplevel_stmt,
    stmt,
    TransactionStmtLegacy,
    AlterEventTrigStmt,
    AlterCollationStmt,
    AlterDatabaseStmt,
    AlterDatabaseSetStmt,
    AlterDefaultPrivilegesStmt,
    AlterDomainStmt,
    AlterEnumStmt,
    AlterExtensionStmt,
    AlterExtensionContentsStmt,
    AlterFdwStmt,
    AlterForeignServerStmt,
    AlterFunctionStmt,
    AlterGroupStmt,
    AlterObjectDependsStmt,
    AlterObjectSchemaStmt,
    AlterOwnerStmt,
    AlterOperatorStmt,
    AlterTypeStmt,
    AlterPolicyStmt,
    AlterSeqStmt,
    AlterSystemStmt,
    AlterTableStmt,
    AlterTblSpcStmt,
    AlterCompositeTypeStmt,
    AlterPublicationStmt,
    AlterRoleSetStmt,
    AlterRoleStmt,
    AlterSubscriptionStmt,
    AlterStatsStmt,
    AlterTSConfigurationStmt,
    AlterTSDictionaryStmt,
    AlterUserMappingStmt,
    AnalyzeStmt,
    CallStmt,
    CheckPointStmt,
    ClosePortalStmt,
    ClusterStmt,
    CommentStmt,
    ConstraintsSetStmt,
    CopyStmt,
    CreateAmStmt,
    CreateAsStmt,
    CreateAssertionStmt,
    CreateCastStmt,
    CreateConversionStmt,
    CreateDomainStmt,
    CreateExtensionStmt,
    CreateFdwStmt,
    CreateForeignServerStmt,
    CreateForeignTableStmt,
    CreateFunctionStmt,
    CreateGroupStmt,
    CreateMatViewStmt,
    CreateOpClassStmt,
    CreateOpFamilyStmt,
    CreatePublicationStmt,
    AlterOpFamilyStmt,
    CreatePolicyStmt,
    CreatePLangStmt,
    CreateSchemaStmt,
    CreateSeqStmt,
    CreateStmt,
    CreateSubscriptionStmt,
    CreateStatsStmt,
    CreateTableSpaceStmt,
    CreateTransformStmt,
    CreateTrigStmt,
    CreateEventTrigStmt,
    CreateRoleStmt,
    CreateUserStmt,
    CreateUserMappingStmt,
    CreatedbStmt,
    DeallocateStmt,
    DeclareCursorStmt,
    DefineStmt,
    DeleteStmt,
    DiscardStmt,
    DoStmt,
    DropCastStmt,
    DropOpClassStmt,
    DropOpFamilyStmt,
    DropOwnedStmt,
    DropStmt,
    DropSubscriptionStmt,
    DropTableSpaceStmt,
    DropTransformStmt,
    DropRoleStmt,
    DropUserMappingStmt,
    DropdbStmt,
    ExecuteStmt,
    ExplainStmt,
    FetchStmt,
    GrantStmt,
    GrantRoleStmt,
    ImportForeignSchemaStmt,
    IndexStmt,
    InsertStmt,
    ListenStmt,
    RefreshMatViewStmt,
    LoadStmt,
    LockStmt,
    MergeStmt,
    NotifyStmt,
    PrepareStmt,
    ReassignOwnedStmt,
    ReindexStmt,
    RemoveAggrStmt,
    RemoveFuncStmt,
    RemoveOperStmt,
    RenameStmt,
    RevokeStmt,
    RevokeRoleStmt,
    RuleStmt,
    SecLabelStmt,
    SelectStmt,
    TransactionStmt,
    TruncateStmt,
    UnlistenStmt,
    UpdateStmt,
    VacuumStmt,
    VariableResetStmt,
    VariableSetStmt,
    VariableShowStmt,
    ViewStmt,
    ColId,
    any_name,
    func_application,
    RoleId,
    opt_with,
    OptRoleList,
    CreateOptRoleElem,
    AlterOptRoleList,
    AlterOptRoleElem,
    Sconst,
    SignedIconst,
    role_list,
    Iconst,
    RoleSpec,
    name,
    opt_in_database,
    SetResetClause,
    add_drop,
    opt_single_name,
    OptSchemaEltList,
    schema_stmt,
    set_rest,
    transaction_mode_list,
    set_rest_more,
    var_name,
    var_list,
    generic_set,
    zone_value,
    opt_encoding,
    NonReservedWord_or_Sconst,
    document_or_content,
    var_value,
    opt_boolean_or_string,
    NumericOnly,
    ConstInterval,
    opt_interval,
    NonReservedWord,
    reset_rest,
    generic_reset,
    constraints_set_list,
    constraints_set_mode,
    qualified_name_list,
    relation_expr,
    alter_table_cmds,
    partition_cmd,
    opt_nowait,
    qualified_name,
    index_partition_cmd,
    alter_table_cmd,
    PartitionBoundSpec,
    opt_concurrently,
    columnDef,
    opt_column,
    alter_column_default,
    reloptions,
    column_storage,
    column_compression,
    generated_when,
    OptParenthesizedSeqOptList,
    alter_identity_column_option_list,
    opt_drop_behavior,
    opt_set_data,
    opt_collate_clause,
    alter_using,
    alter_generic_options,
    TableConstraint,
    ConstraintAttributeSpec,
    replica_identity,
    a_expr,
    reloption_list,
    reloption_elem,
    ColLabel,
    def_arg,
    alter_identity_column_option,
    SeqOptElem,
    hash_partbound,
    expr_list,
    hash_partbound_elem,
    alter_type_cmds,
    alter_type_cmd,
    TableFuncElement,
    cursor_name,
    opt_binary,
    opt_column_list,
    copy_from,
    opt_program,
    copy_file_name,
    copy_delimiter,
    copy_options,
    where_clause,
    PreparableStmt,
    copy_opt_list,
    copy_generic_opt_list,
    copy_opt_item,
    opt_as,
    columnList,
    opt_using,
    copy_generic_opt_elem,
    copy_generic_opt_arg,
    copy_generic_opt_arg_list,
    copy_generic_opt_arg_list_item,
    OptTemp,
    OptTableElementList,
    OptInherit,
    OptPartitionSpec,
    table_access_method_clause,
    OptWith,
    OnCommitOption,
    OptTableSpace,
    OptTypedTableElementList,
    TableElementList,
    TypedTableElementList,
    TableElement,
    TypedTableElement,
    TableLikeClause,
    columnOptions,
    opt_column_storage,
    opt_column_compression,
    create_generic_options,
    ColQualList,
    ColConstraint,
    ColConstraintElem,
    ConstraintAttr,
    opt_unique_null_treatment,
    opt_definition,
    OptConsTableSpace,
    opt_no_inherit,
    b_expr,
    key_match,
    key_actions,
    TableLikeOptionList,
    TableLikeOption,
    ConstraintElem,
    opt_c_include,
    ExistingIndex,
    access_method_clause,
    ExclusionConstraintList,
    OptWhereClause,
    columnElem,
    ExclusionConstraintElem,
    index_elem,
    any_operator,
    key_update,
    key_delete,
    key_action,
    PartitionSpec,
    part_params,
    part_elem,
    opt_collate,
    opt_qualified_name,
    func_expr_windowless,
    opt_name_list,
    stats_params,
    from_list,
    stats_param,
    create_as_target,
    opt_with_data,
    OptNoLog,
    create_mv_target,
    opt_reloptions,
    OptSeqOptList,
    SeqOptList,
    SimpleTypename,
    opt_by,
    NumericOnly_list,
    opt_or_replace,
    opt_trusted,
    opt_procedural,
    handler_name,
    opt_inline_handler,
    opt_validator,
    attrs,
    validator_clause,
    OptTableSpaceOwner,
    create_extension_opt_list,
    create_extension_opt_item,
    alter_extension_opt_list,
    alter_extension_opt_item,
    object_type_name,
    object_type_any_name,
    aggregate_with_argtypes,
    function_with_argtypes,
    operator_with_argtypes,
    opt_fdw_options,
    fdw_option,
    fdw_options,
    generic_option_list,
    generic_option_elem,
    alter_generic_option_list,
    alter_generic_option_elem,
    generic_option_name,
    generic_option_arg,
    opt_type,
    opt_foreign_server_version,
    foreign_server_version,
    import_qualification,
    import_qualification_type,
    relation_expr_list,
    auth_ident,
    RowSecurityDefaultPermissive,
    RowSecurityDefaultForCmd,
    RowSecurityDefaultToRole,
    RowSecurityOptionalExpr,
    RowSecurityOptionalWithCheck,
    RowSecurityOptionalToRole,
    row_security_cmd,
    am_type,
    TriggerActionTime,
    TriggerEvents,
    TriggerReferencing,
    TriggerForSpec,
    TriggerWhen,
    FUNCTION_or_PROCEDURE,
    func_name,
    TriggerFuncArgs,
    OptConstrFromTable,
    TriggerOneEvent,
    TriggerTransitions,
    TriggerTransition,
    TransitionOldOrNew,
    TransitionRowOrTable,
    TransitionRelName,
    TriggerForOptEach,
    TriggerForType,
    TriggerFuncArg,
    ConstraintAttributeElem,
    event_trigger_when_list,
    event_trigger_when_item,
    event_trigger_value_list,
    enable_trigger,
    aggr_args,
    definition,
    old_aggr_definition,
    OptTableFuncElementList,
    opt_enum_val_list,
    def_list,
    def_elem,
    func_type,
    reserved_keyword,
    qual_all_Op,
    old_aggr_list,
    old_aggr_elem,
    enum_val_list,
    opt_if_not_exists,
    opt_default,
    opt_opfamily,
    opclass_item_list,
    opclass_item,
    opclass_purpose,
    opt_recheck,
    type_list,
    opclass_drop_list,
    opclass_drop,
    any_name_list,
    drop_type_name,
    name_list,
    object_type_name_on_any_name,
    type_name_list,
    attr_name,
    opt_table,
    opt_restart_seqs,
    comment_text,
    opt_provider,
    security_label,
    fetch_args,
    from_in,
    opt_from_in,
    privileges,
    privilege_target,
    grantee_list,
    opt_grant_grant_option,
    opt_granted_by,
    privilege_list,
    privilege,
    parameter_name,
    parameter_name_list,
    function_with_argtypes_list,
    grantee,
    grant_role_opt_list,
    grant_role_opt,
    grant_role_opt_value,
    DefACLOptionList,
    DefACLAction,
    DefACLOption,
    defacl_privilege_target,
    opt_unique,
    index_params,
    opt_include,
    opt_asc_desc,
    opt_nulls_order,
    index_elem_options,
    index_including_params,
    func_args_with_defaults,
    func_return,
    opt_createfunc_opt_list,
    opt_routine_body,
    table_func_column_list,
    func_args_list,
    func_arg,
    func_args,
    type_func_name_keyword,
    indirection,
    func_args_with_defaults_list,
    func_arg_with_default,
    arg_class,
    param_name,
    type_function_name,
    aggr_args_list,
    aggr_arg,
    aggregate_with_argtypes_list,
    createfunc_opt_list,
    createfunc_opt_item,
    FunctionSetResetClause,
    func_as,
    transform_type_list,
    common_func_opt_item,
    ReturnStmt,
    routine_body_stmt_list,
    routine_body_stmt,
    table_func_column,
    alterfunc_opt_list,
    opt_restrict,
    operator_with_argtypes_list,
    all_Op,
    oper_argtypes,
    dostmt_opt_list,
    dostmt_opt_item,
    cast_context,
    opt_if_exists,
    transform_element_list,
    opt_reindex_option_list,
    reindex_target_relation,
    reindex_target_all,
    utility_option_list,
    opt_no,
    operator_def_list,
    operator_def_elem,
    operator_def_arg,
    pub_obj_list,
    extended_relation_expr,
    PublicationObjSpec,
    event,
    opt_instead,
    RuleActionList,
    RuleActionStmt,
    RuleActionMulti,
    RuleActionStmtOrEmpty,
    notify_payload,
    opt_transaction,
    opt_transaction_chain,
    transaction_mode_list_or_empty,
    iso_level,
    transaction_mode_item,
    opt_check_option,
    file_name,
    createdb_opt_list,
    createdb_opt_items,
    createdb_opt_item,
    createdb_opt_name,
    opt_equal,
    drop_option_list,
    drop_option,
    any_with,
    opt_verbose,
    cluster_index_specification,
    opt_full,
    opt_freeze,
    opt_analyze,
    opt_vacuum_relation_list,
    analyze_keyword,
    utility_option_elem,
    utility_option_name,
    utility_option_arg,
    vacuum_relation,
    vacuum_relation_list,
    ExplainableStmt,
    prep_type_clause,
    execute_param_clause,
    opt_with_clause,
    insert_target,
    insert_rest,
    opt_on_conflict,
    returning_clause,
    override_kind,
    insert_column_list,
    insert_column_item,
    opt_indirection,
    opt_conf_expr,
    set_clause_list,
    target_list,
    relation_expr_opt_alias,
    using_clause,
    where_or_current_clause,
    opt_lock,
    lock_type,
    from_clause,
    set_clause,
    set_target,
    set_target_list,
    table_ref,
    merge_when_list,
    merge_when_clause,
    opt_merge_when_condition,
    merge_update,
    merge_delete,
    merge_insert,
    merge_values_clause,
    cursor_options,
    opt_hold,
    select_no_parens,
    select_with_parens,
    simple_select,
    select_clause,
    sort_clause,
    opt_sort_clause,
    for_locking_clause,
    opt_select_limit,
    select_limit,
    opt_for_locking_clause,
    with_clause,
    opt_all_clause,
    opt_target_list,
    into_clause,
    group_clause,
    having_clause,
    window_clause,
    distinct_clause,
    values_clause,
    set_quantifier,
    cte_list,
    common_table_expr,
    opt_materialized,
    opt_search_clause,
    opt_cycle_clause,
    AexprConst,
    OptTempTableName,
    sortby_list,
    sortby,
    limit_clause,
    offset_clause,
    select_limit_value,
    select_offset_value,
    first_or_next,
    select_fetch_first_value,
    row_or_rows,
    c_expr,
    I_or_F_const,
    group_by_list,
    group_by_item,
    empty_grouping_set,
    cube_clause,
    rollup_clause,
    grouping_sets_clause,
    for_locking_items,
    for_locking_item,
    for_locking_strength,
    locked_rels_list,
    opt_nowait_or_skip,
    opt_alias_clause,
    tablesample_clause,
    func_table,
    func_alias_clause,
    xmltable,
    joined_table,
    alias_clause,
    join_type,
    join_qual,
    TableFuncElementList,
    opt_outer,
    opt_alias_clause_for_join_using,
    opt_repeatable_clause,
    opt_ordinality,
    rowsfrom_list,
    opt_col_def_list,
    rowsfrom_item,
    xmlexists_argument,
    xmltable_column_list,
    xml_namespace_list,
    xmltable_column_el,
    xmltable_column_option_list,
    xmltable_column_option_el,
    xml_namespace_el,
    opt_array_bounds,
    GenericType,
    Numeric,
    Bit,
    Character,
    ConstDatetime,
    ConstBit,
    ConstCharacter,
    opt_type_modifiers,
    opt_float,
    BitWithLength,
    BitWithoutLength,
    opt_varying,
    CharacterWithLength,
    CharacterWithoutLength,
    character,
    opt_timezone,
    interval_second,
    qual_Op,
    row,
    opt_asymmetric,
    in_expr,
    subquery_Op,
    sub_type,
    unicode_normal_form,
    json_predicate_type_constraint,
    json_key_uniqueness_constraint_opt,
    columnref,
    case_expr,
    func_expr,
    array_expr,
    explicit_row,
    implicit_row,
    func_arg_list,
    func_arg_expr,
    within_group_clause,
    filter_clause,
    over_clause,
    json_aggregate_func,
    func_expr_common_subexpr,
    extract_list,
    overlay_list,
    func_arg_list_opt,
    position_list,
    substr_list,
    trim_list,
    xml_attributes,
    xml_attribute_list,
    xml_whitespace_option,
    xml_root_version,
    opt_xml_root_standalone,
    xml_indent_option,
    json_name_and_value_list,
    json_object_constructor_null_clause_opt,
    json_output_clause_opt,
    json_value_expr_list,
    json_array_constructor_null_clause_opt,
    json_format_clause_opt,
    xml_attribute_el,
    xml_passing_mech,
    window_definition_list,
    window_definition,
    window_specification,
    opt_existing_window_name,
    opt_partition_clause,
    opt_frame_clause,
    frame_extent,
    opt_window_exclusion_clause,
    frame_bound,
    MathOp,
    array_expr_list,
    extract_arg,
    case_arg,
    when_clause_list,
    case_default,
    when_clause,
    opt_slice_bound,
    indirection_el,
    json_encoding_clause_opt,
    json_name_and_value,
    json_value_expr,
    json_array_aggregate_order_by_clause_opt,
    target_el,
    BareColLabel,
    ConstTypename,
    opt_distinct_clause,
    plassign_target,
    plassign_equals,
    unreserved_keyword,
    col_name_keyword,
    bare_label_keyword,
    Dollaraccept,
    parse_toplevel,
    C_COMMENT,
    SQL_COMMENT,
    Whitespace,
    Root,
}
