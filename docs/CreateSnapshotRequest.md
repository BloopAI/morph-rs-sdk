# CreateSnapshotRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image_id** | Option<**String**> |  | [optional]
**readiness_check** | Option<[**models::TimeoutCheck**](TimeoutCheck.md)> |  | [optional]
**vcpus** | Option<**i32**> | Number of vCPUs | [optional][default to 1]
**memory** | Option<**i32**> | Memory in MB | [optional][default to 128]
**disk_size** | Option<**i32**> | Size of the disk in MB | [optional][default to 700]
**digest** | Option<**String**> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


