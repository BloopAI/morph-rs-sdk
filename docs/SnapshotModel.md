# SnapshotModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the snapshot, like snapshot_xxxx | 
**object** | Option<**String**> | Object type, always 'snapshot' | [optional][default to Snapshot]
**created** | **i32** | Unix timestamp of when the snapshot was created | 
**status** | [**models::SnapshotStatus**](SnapshotStatus.md) | Status of the snapshot | 
**spec** | [**models::ResourceSpec**](ResourceSpec.md) |  | 
**refs** | [**models::SnapshotRefs**](SnapshotRefs.md) |  | 
**digest** | Option<**String**> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> | User provided metadata for the snapshot | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


