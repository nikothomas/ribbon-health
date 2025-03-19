# GetEligibility200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**parameters** | [**models::GetEligibility200ResponseParameters**](getEligibility_200_response_parameters.md) |  | 
**status** | [**models::GetEligibility200ResponseStatus**](getEligibility_200_response_status.md) |  | 
**request_id** | **String** | A unique identifier for the given response which Ribbon can use to find it in our logs during support requests | 
**plan_info** | Option<[**models::GetEligibility200ResponsePlanInfo**](getEligibility_200_response_plan_info.md)> |  | [optional]
**deductible_detail** | Option<[**models::GetEligibility200ResponseDeductibleDetail**](getEligibility_200_response_deductible_detail.md)> |  | [optional]
**out_of_pocket_detail** | Option<[**models::GetEligibility200ResponseOutOfPocketDetail**](getEligibility_200_response_out_of_pocket_detail.md)> |  | [optional]
**primary_care_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**dme_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**oncology_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**vision_optometry_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**physical_therapy_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**specialist_office_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**mental_health_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**surgical_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**urgent_care_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**diagnostic_lab_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**asc_facility_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**chiropractic_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**mri_ct_scan_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**x_ray_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**speech_therapy_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**occupational_therapy_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**emergency_medical_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**wellness_or_routine_visit_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**podiatry_office_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**outpatient_professional_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**anesthesia_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**substance_abuse_professional_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**substance_abuse_in_patient_facility_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**substance_abuse_out_patient_facility_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**flu_vaccination_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**hospital_inpatient_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**pharmacy_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**hospital_outpatient_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**telemedicine_primary_care_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**telemedicine_specialist_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**telemedicine_urgent_care_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**telemedicine_physical_therapy_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**telemedicine_mental_health_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**psychotherapy_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**snf_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**snf_room_board_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**home_health_care_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**hospice_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]
**professional_physician_visit_inpatient_summary** | Option<[**models::GetEligibility200ResponsePrimaryCareSummary**](getEligibility_200_response_primary_care_summary.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


