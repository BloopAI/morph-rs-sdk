# \UsageApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_usage_user_usage_get**](UsageApi.md#get_user_usage_user_usage_get) | **GET** /user/usage | Get User Usage



## get_user_usage_user_usage_get

> models::UserUsageOverviewModel get_user_usage_user_usage_get(interval)
Get User Usage

Get the current user's usage overview

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval** | Option<**String**> |  |  |[default to 3h]

### Return type

[**models::UserUsageOverviewModel**](UserUsageOverviewModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

