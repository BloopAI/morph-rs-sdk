# \ApiKeysApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key_user_api_key_post**](ApiKeysApi.md#create_api_key_user_api_key_post) | **POST** /user/api-key | Create Api Key
[**delete_api_key_user_api_key_api_key_id_delete**](ApiKeysApi.md#delete_api_key_user_api_key_api_key_id_delete) | **DELETE** /user/api-key/{api_key_id} | Delete Api Key
[**list_api_keys_user_api_key_get**](ApiKeysApi.md#list_api_keys_user_api_key_get) | **GET** /user/api-key | List Api Keys



## create_api_key_user_api_key_post

> models::CreateApiKeyResponse create_api_key_user_api_key_post()
Create Api Key

Create a new API key for the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CreateApiKeyResponse**](CreateAPIKeyResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key_user_api_key_api_key_id_delete

> delete_api_key_user_api_key_api_key_id_delete(api_key_id)
Delete Api Key

Delete an API key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_api_keys_user_api_key_get

> models::ApiKeyModelCollection list_api_keys_user_api_key_get()
List Api Keys

List all active API keys for the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiKeyModelCollection**](APIKeyModelCollection.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

