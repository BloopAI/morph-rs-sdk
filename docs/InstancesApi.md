# \InstancesApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**boot_instance_snapshot_snapshot_id_boot_post**](InstancesApi.md#boot_instance_snapshot_snapshot_id_boot_post) | **POST** /snapshot/{snapshot_id}/boot | Boot Instance
[**branch_instance_instance_instance_id_branch_post**](InstancesApi.md#branch_instance_instance_instance_id_branch_post) | **POST** /instance/{instance_id}/branch | Branch Instance
[**exec_instance_instance_id_exec_post**](InstancesApi.md#exec_instance_instance_id_exec_post) | **POST** /instance/{instance_id}/exec | Exec
[**expose_http_service_instance_instance_id_http_post**](InstancesApi.md#expose_http_service_instance_instance_id_http_post) | **POST** /instance/{instance_id}/http | Expose Http Service
[**get_instance_instance_instance_id_get**](InstancesApi.md#get_instance_instance_instance_id_get) | **GET** /instance/{instance_id} | Get Instance
[**get_ssh_key_instance_instance_id_ssh_key_get**](InstancesApi.md#get_ssh_key_instance_instance_id_ssh_key_get) | **GET** /instance/{instance_id}/ssh/key | Get Ssh Key
[**hide_http_service_instance_instance_id_http_service_name_delete**](InstancesApi.md#hide_http_service_instance_instance_id_http_service_name_delete) | **DELETE** /instance/{instance_id}/http/{service_name} | Hide Http Service
[**list_instances_instance_get**](InstancesApi.md#list_instances_instance_get) | **GET** /instance | List Instances
[**pause_instance_instance_instance_id_pause_post**](InstancesApi.md#pause_instance_instance_instance_id_pause_post) | **POST** /instance/{instance_id}/pause | Pause Instance
[**reboot_instance_instance_instance_id_reboot_post**](InstancesApi.md#reboot_instance_instance_instance_id_reboot_post) | **POST** /instance/{instance_id}/reboot | Reboot Instance
[**resume_instance_instance_instance_id_resume_post**](InstancesApi.md#resume_instance_instance_instance_id_resume_post) | **POST** /instance/{instance_id}/resume | Resume Instance
[**rotate_ssh_key_instance_instance_id_ssh_key_post**](InstancesApi.md#rotate_ssh_key_instance_instance_id_ssh_key_post) | **POST** /instance/{instance_id}/ssh/key | Rotate Ssh Key
[**set_instance_metadata_instance_instance_id_metadata_post**](InstancesApi.md#set_instance_metadata_instance_instance_id_metadata_post) | **POST** /instance/{instance_id}/metadata | Set Instance Metadata
[**snapshot_instance_instance_instance_id_snapshot_post**](InstancesApi.md#snapshot_instance_instance_instance_id_snapshot_post) | **POST** /instance/{instance_id}/snapshot | Snapshot Instance
[**start_instance_instance_post**](InstancesApi.md#start_instance_instance_post) | **POST** /instance | Start Instance
[**stop_instance_instance_instance_id_delete**](InstancesApi.md#stop_instance_instance_instance_id_delete) | **DELETE** /instance/{instance_id} | Stop Instance
[**update_instance_ttl_instance_instance_id_ttl_post**](InstancesApi.md#update_instance_ttl_instance_instance_id_ttl_post) | **POST** /instance/{instance_id}/ttl | Update Instance Ttl
[**update_instance_wake_on_instance_instance_id_wake_on_post**](InstancesApi.md#update_instance_wake_on_instance_instance_id_wake_on_post) | **POST** /instance/{instance_id}/wake-on | Update Instance Wake On



## boot_instance_snapshot_snapshot_id_boot_post

> models::InstanceModel boot_instance_snapshot_snapshot_id_boot_post(snapshot_id, boot_instance_request)
Boot Instance

Boot an instance from a snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** |  | [required] |
**boot_instance_request** | [**BootInstanceRequest**](BootInstanceRequest.md) |  | [required] |

### Return type

[**models::InstanceModel**](InstanceModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## branch_instance_instance_instance_id_branch_post

> models::BranchInstanceResponse branch_instance_instance_instance_id_branch_post(instance_id, count, digest, branch_instance_request)
Branch Instance

Branch an instance into multiple instances creating a new snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **String** |  | [required] |
**count** | Option<**i32**> |  |  |[default to 1]
**digest** | Option<**String**> |  |  |
**branch_instance_request** | Option<[**BranchInstanceRequest**](BranchInstanceRequest.md)> |  |  |

### Return type

[**models::BranchInstanceResponse**](BranchInstanceResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exec_instance_instance_id_exec_post

> models::ExecResponse exec_instance_instance_id_exec_post(instance_id, exec_request)
Exec

Execute a shell command inside a VM via SSH (AsyncSSH backend).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **String** |  | [required] |
**exec_request** | [**ExecRequest**](ExecRequest.md) |  | [required] |

### Return type

[**models::ExecResponse**](ExecResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## expose_http_service_instance_instance_id_http_post

> models::InstanceModel expose_http_service_instance_instance_id_http_post(instance_id, expose_http_service_request)
Expose Http Service

Expose an HTTP service on an instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **String** |  | [required] |
**expose_http_service_request** | [**ExposeHttpServiceRequest**](ExposeHttpServiceRequest.md) |  | [required] |

### Return type

[**models::InstanceModel**](InstanceModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instance_instance_instance_id_get

> models::InstanceModel get_instance_instance_instance_id_get(instance_id)
Get Instance

Get an instance by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **String** |  | [required] |

### Return type

[**models::InstanceModel**](InstanceModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ssh_key_instance_instance_id_ssh_key_get

> models::InstanceSshKey get_ssh_key_instance_instance_id_ssh_key_get(instance_id)
Get Ssh Key

Get the SSH key pair for an instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **String** |  | [required] |

### Return type

[**models::InstanceSshKey**](InstanceSshKey.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hide_http_service_instance_instance_id_http_service_name_delete

> models::InstanceModel hide_http_service_instance_instance_id_http_service_name_delete(instance_id, service_name)
Hide Http Service

Hide an HTTP service on an instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **String** |  | [required] |
**service_name** | **String** |  | [required] |

### Return type

[**models::InstanceModel**](InstanceModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_instances_instance_get

> models::InstanceModelCollection list_instances_instance_get(metadata)
List Instances

List instances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metadata** | Option<[**std::collections::HashMap<String, String>**](String.md)> | metadata[...] query parameters for filtering instances |  |

### Return type

[**models::InstanceModelCollection**](InstanceModelCollection.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pause_instance_instance_instance_id_pause_post

> models::InstanceModel pause_instance_instance_instance_id_pause_post(instance_id, no_snapshot)
Pause Instance

Pause an instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **String** |  | [required] |
**no_snapshot** | Option<**bool**> |  |  |

### Return type

[**models::InstanceModel**](InstanceModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reboot_instance_instance_instance_id_reboot_post

> models::InstanceModel reboot_instance_instance_instance_id_reboot_post(instance_id)
Reboot Instance

Restart an instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **String** |  | [required] |

### Return type

[**models::InstanceModel**](InstanceModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_instance_instance_instance_id_resume_post

> models::InstanceModel resume_instance_instance_instance_id_resume_post(instance_id)
Resume Instance

Resume a paused instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **String** |  | [required] |

### Return type

[**models::InstanceModel**](InstanceModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_ssh_key_instance_instance_id_ssh_key_post

> models::InstanceSshKey rotate_ssh_key_instance_instance_id_ssh_key_post(instance_id)
Rotate Ssh Key

Rotate the SSH key pair for an instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **String** |  | [required] |

### Return type

[**models::InstanceSshKey**](InstanceSshKey.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_instance_metadata_instance_instance_id_metadata_post

> models::InstanceModel set_instance_metadata_instance_instance_id_metadata_post(instance_id, request_body)
Set Instance Metadata

Set metadata for an instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **String** |  | [required] |
**request_body** | [**std::collections::HashMap<String, String>**](String.md) |  | [required] |

### Return type

[**models::InstanceModel**](InstanceModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## snapshot_instance_instance_instance_id_snapshot_post

> models::SnapshotModel snapshot_instance_instance_instance_id_snapshot_post(instance_id, digest, snapshot_instance_request)
Snapshot Instance

Create a snapshot from an instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **String** |  | [required] |
**digest** | Option<**String**> |  |  |
**snapshot_instance_request** | Option<[**SnapshotInstanceRequest**](SnapshotInstanceRequest.md)> |  |  |

### Return type

[**models::SnapshotModel**](SnapshotModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_instance_instance_post

> models::InstanceModel start_instance_instance_post(snapshot_id, start_instance_request)
Start Instance

Start a new instance from a snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** |  | [required] |
**start_instance_request** | Option<[**StartInstanceRequest**](StartInstanceRequest.md)> |  |  |

### Return type

[**models::InstanceModel**](InstanceModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_instance_instance_instance_id_delete

> stop_instance_instance_instance_id_delete(instance_id)
Stop Instance

Stop and delete an instance by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance_ttl_instance_instance_id_ttl_post

> models::InstanceModel update_instance_ttl_instance_instance_id_ttl_post(instance_id, update_ttl_request)
Update Instance Ttl

Update the TTL (Time To Live) for an instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **String** |  | [required] |
**update_ttl_request** | [**UpdateTtlRequest**](UpdateTtlRequest.md) |  | [required] |

### Return type

[**models::InstanceModel**](InstanceModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_instance_wake_on_instance_instance_id_wake_on_post

> models::InstanceModel update_instance_wake_on_instance_instance_id_wake_on_post(instance_id, update_wake_request)
Update Instance Wake On

Update the wake-on-request settings for an instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_id** | **String** |  | [required] |
**update_wake_request** | [**UpdateWakeRequest**](UpdateWakeRequest.md) |  | [required] |

### Return type

[**models::InstanceModel**](InstanceModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

