# \SnapshotsApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_snapshot_snapshot_post**](SnapshotsApi.md#create_snapshot_snapshot_post) | **POST** /snapshot | Create Snapshot
[**delete_snapshot_snapshot_snapshot_id_delete**](SnapshotsApi.md#delete_snapshot_snapshot_snapshot_id_delete) | **DELETE** /snapshot/{snapshot_id} | Delete Snapshot
[**get_snapshot_snapshot_snapshot_id_get**](SnapshotsApi.md#get_snapshot_snapshot_snapshot_id_get) | **GET** /snapshot/{snapshot_id} | Get Snapshot
[**list_snapshots_snapshot_get**](SnapshotsApi.md#list_snapshots_snapshot_get) | **GET** /snapshot | List Snapshots
[**set_snapshot_metadata_snapshot_snapshot_id_metadata_post**](SnapshotsApi.md#set_snapshot_metadata_snapshot_snapshot_id_metadata_post) | **POST** /snapshot/{snapshot_id}/metadata | Set Snapshot Metadata



## create_snapshot_snapshot_post

> models::SnapshotModel create_snapshot_snapshot_post(create_snapshot_request)
Create Snapshot

Create a snapshot from an image.  Snapshots are point-in-time copies of a running instance that can be used to create new instances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_snapshot_request** | [**CreateSnapshotRequest**](CreateSnapshotRequest.md) |  | [required] |

### Return type

[**models::SnapshotModel**](SnapshotModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_snapshot_snapshot_snapshot_id_delete

> delete_snapshot_snapshot_snapshot_id_delete(snapshot_id)
Delete Snapshot

Delete a snapshot by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_snapshot_snapshot_snapshot_id_get

> models::SnapshotModel get_snapshot_snapshot_snapshot_id_get(snapshot_id)
Get Snapshot

Get a snapshot by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** |  | [required] |

### Return type

[**models::SnapshotModel**](SnapshotModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_snapshots_snapshot_get

> models::SnapshotModelCollection list_snapshots_snapshot_get(digest, metadata)
List Snapshots

List snapshots.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**digest** | Option<**String**> |  |  |
**metadata** | Option<[**std::collections::HashMap<String, String>**](String.md)> | metadata[...] query parameters for filtering snapshots |  |

### Return type

[**models::SnapshotModelCollection**](SnapshotModelCollection.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_snapshot_metadata_snapshot_snapshot_id_metadata_post

> models::SnapshotModel set_snapshot_metadata_snapshot_snapshot_id_metadata_post(snapshot_id, request_body)
Set Snapshot Metadata

Set metadata for a snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** |  | [required] |
**request_body** | [**std::collections::HashMap<String, String>**](String.md) |  | [required] |

### Return type

[**models::SnapshotModel**](SnapshotModel.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

