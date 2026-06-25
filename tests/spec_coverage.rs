use kalshi_pmx::ws::Channel;

#[test]
fn openapi_operation_ids_are_accounted_for() {
    let Some(openapi) = repo_spec("openapi.yaml") else {
        eprintln!("skipping OpenAPI coverage check: repo-only specs are not packaged");
        return;
    };
    let operations = openapi
        .lines()
        .filter_map(|line| line.trim().strip_prefix("operationId: "))
        .map(str::to_owned)
        .collect::<Vec<_>>();

    let supported = [
        "GetExchangeStatus",
        "GetExchangeAnnouncements",
        "GetSeriesFeeChanges",
        "GetExchangeSchedule",
        "GetUserDataTimestamp",
        "GetMarketCandlesticks",
        "GetTrades",
        "GetMarketOrderbook",
        "GetMarketOrderbooks",
        "GetSeries",
        "GetSeriesList",
        "GetMarkets",
        "GetMarket",
        "BatchGetMarketCandlesticks",
        "GetMarketCandlesticksByEvent",
        "GetEvents",
        "GetMultivariateEvents",
        "GetEventFeeChanges",
        "GetEvent",
        "GetEventMetadata",
        "GetEventForecastPercentilesHistory",
        "GetOrders",
        "GetOrder",
        "GetOrderQueuePositions",
        "GetOrderQueuePosition",
        "CreateOrderV2",
        "BatchCreateOrdersV2",
        "BatchCancelOrdersV2",
        "CancelOrderV2",
        "AmendOrderV2",
        "DecreaseOrderV2",
        "GetOrderGroups",
        "CreateOrderGroup",
        "GetOrderGroup",
        "DeleteOrderGroup",
        "ResetOrderGroup",
        "TriggerOrderGroup",
        "UpdateOrderGroupLimit",
        "GetBalance",
        "CreateSubaccount",
        "ApplySubaccountTransfer",
        "GetSubaccountBalances",
        "GetSubaccountTransfers",
        "UpdateSubaccountNetting",
        "GetSubaccountNetting",
        "GetPositions",
        "GetSettlements",
        "GetDeposits",
        "GetWithdrawals",
        "GetPortfolioRestingOrderTotalValue",
        "GetFills",
        "GetCommunicationsID",
        "GetBlockTradeProposals",
        "ProposeBlockTrade",
        "AcceptBlockTradeProposal",
        "GetRFQs",
        "CreateRFQ",
        "GetRFQ",
        "DeleteRFQ",
        "GetQuotes",
        "CreateQuote",
        "GetQuote",
        "DeleteQuote",
        "AcceptQuote",
        "ConfirmQuote",
        "GetApiKeys",
        "CreateApiKey",
        "GenerateApiKey",
        "DeleteApiKey",
        "GetAccountApiLimits",
        "UpgradeAccountApiUsageLevel",
        "GetAccountApiUsageLevelVolumeProgress",
        "GetAccountEndpointCosts",
        "GetTagsForSeriesCategories",
        "GetFiltersForSports",
        "GetLiveDataByMilestone",
        "GetLiveData",
        "GetLiveDatas",
        "GetGameStats",
        "GetStructuredTargets",
        "GetStructuredTarget",
        "GetMilestone",
        "GetMilestones",
        "GetMultivariateEventCollection",
        "CreateMarketInMultivariateEventCollection",
        "GetMultivariateEventCollections",
        "LookupTickersForMarketInMultivariateEventCollection",
        "GetMultivariateEventCollectionLookupHistory",
        "GetIncentivePrograms",
        "GetFCMOrders",
        "GetFCMPositions",
        "GetHistoricalCutoff",
        "GetMarketCandlesticksHistorical",
        "GetFillsHistorical",
        "GetHistoricalOrders",
        "GetTradesHistorical",
        "GetHistoricalMarkets",
        "GetHistoricalMarket",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();

    assert_eq!(operations, supported);

    let typed_method_count = include_str!("../src/typed.rs")
        .matches("pub async fn ")
        .count();
    assert_eq!(typed_method_count, operations.len());
}

#[test]
fn openapi_component_schemas_are_generated() {
    let generated_type_count = include_str!("../src/generated.rs")
        .lines()
        .filter(|line| {
            line.starts_with("pub struct ")
                || line.starts_with("pub type ")
                || line.starts_with("pub enum ")
        })
        .count();

    assert_eq!(generated_type_count, 181);
}

#[test]
fn asyncapi_channels_are_accounted_for() {
    let Some(asyncapi) = repo_spec("asyncapi.yaml") else {
        eprintln!("skipping AsyncAPI coverage check: repo-only specs are not packaged");
        return;
    };
    let channels = asyncapi_channels(&asyncapi);
    assert_eq!(
        channels,
        [
            "root",
            "control_frames",
            "orderbook_delta",
            "ticker",
            "trade",
            "fill",
            "market_positions",
            "market_lifecycle_v2",
            "multivariate_market_lifecycle",
            "multivariate",
            "communications",
            "order_group_updates",
            "user_orders",
            "cfbenchmarks_value",
        ]
    );

    let subscription_channels = [
        Channel::OrderbookDelta,
        Channel::Ticker,
        Channel::Trade,
        Channel::Fill,
        Channel::MarketPositions,
        Channel::MarketLifecycleV2,
        Channel::MultivariateMarketLifecycle,
        Channel::Multivariate,
        Channel::Communications,
        Channel::OrderGroupUpdates,
        Channel::UserOrders,
        Channel::CfBenchmarksValue,
    ]
    .map(|channel| serde_json::to_value(channel).unwrap());

    assert_eq!(
        subscription_channels,
        [
            "orderbook_delta",
            "ticker",
            "trade",
            "fill",
            "market_positions",
            "market_lifecycle_v2",
            "multivariate_market_lifecycle",
            "multivariate",
            "communications",
            "order_group_updates",
            "user_orders",
            "cfbenchmarks_value",
        ]
        .map(serde_json::Value::from)
    );
}

#[test]
fn rest_namespaces_stay_typed_and_high_level() {
    for entry in
        std::fs::read_dir(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/rest"))
            .unwrap()
    {
        let path = entry.unwrap().path();
        if path.file_name().and_then(|name| name.to_str()) == Some("mod.rs") {
            continue;
        }

        let source = std::fs::read_to_string(&path).unwrap();
        assert!(
            !source.contains("raw_"),
            "{} should not call hidden raw request helpers",
            path.display()
        );
        assert!(
            !source.contains("request_json"),
            "{} should delegate through the typed OpenAPI layer",
            path.display()
        );
        assert!(
            source.contains(".typed()"),
            "{} should delegate to generated typed operations",
            path.display()
        );
    }
}

fn repo_spec(file_name: &str) -> Option<String> {
    std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("specs")
            .join(file_name),
    )
    .ok()
}

fn asyncapi_channels(spec: &str) -> Vec<&str> {
    let mut channels = Vec::new();
    let mut in_channels = false;

    for line in spec.lines() {
        if line == "channels:" {
            in_channels = true;
            continue;
        }

        if line == "operations:" {
            break;
        }

        if in_channels && line.starts_with("  ") && !line.starts_with("    ") && line.ends_with(':')
        {
            channels.push(line.trim().trim_end_matches(':'));
        }
    }

    channels
}
