# InstanceModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**object** | Option<**String**> |  | [optional][default to Instance]
**created** | **i32** |  | 
**status** | Option<[**models::InstanceStatus**](InstanceStatus.md)> |  | [optional][default to Pending]
**spec** | [**models::ResourceSpec**](ResourceSpec.md) |  | 
**refs** | [**models::InstanceRefs**](InstanceRefs.md) |  | 
**networking** | [**models::InstanceNetworking**](InstanceNetworking.md) |  | 
**metadata** | Option<**std::collections::HashMap<String, String>**> | User provided metadata for the instance | [optional]
**ttl** | [**models::InstanceTtl**](InstanceTTL.md) | Time to live settings for the instance. If not set, the instance will not expire. | 
**wake_on** | [**models::InstanceWakeOn**](InstanceWakeOn.md) | Wake on settings for the instance. If not set, the instance will not wake on access. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


