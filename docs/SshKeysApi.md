# \SshKeysApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_ssh_key_user_ssh_key_get**](SshKeysApi.md#get_user_ssh_key_user_ssh_key_get) | **GET** /user/ssh-key | Get User Ssh Key
[**update_user_ssh_key_user_ssh_key_put**](SshKeysApi.md#update_user_ssh_key_user_ssh_key_put) | **PUT** /user/ssh-key | Update User Ssh Key



## get_user_ssh_key_user_ssh_key_get

> models::UserSshKeyModel get_user_ssh_key_user_ssh_key_get()
Get User Ssh Key

Get the current user's SSH public key.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserSshKeyModel**](UserSSHKeyModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_ssh_key_user_ssh_key_put

> models::UserSshKeyModel update_user_ssh_key_user_ssh_key_put(update_ssh_key_request)
Update User Ssh Key

Update the current user's SSH public key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_ssh_key_request** | [**UpdateSshKeyRequest**](UpdateSshKeyRequest.md) |  | [required] |

### Return type

[**models::UserSshKeyModel**](UserSSHKeyModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

