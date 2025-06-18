<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST - login success</name>
   <tag></tag>
   <elementGuidId>a076ff2e-a827-48c4-b19a-9d1ebb2302d7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;email\&quot;: \&quot;eve.holt@reqres.in\&quot;,\n  \&quot;password\&quot;: \&quot;cityslicka\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>00b44287-5dd9-473f-99c5-d4dd966070fe</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-api-key</name>
      <type>Main</type>
      <value>reqres-free-v1</value>
      <webElementGuid>7ac57af2-7eaf-48f6-ac6f-9ac0005e4e6a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.2.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://reqres.in/api/login</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper
import static org.assertj.core.api.Assertions.*

// Kirim request POST login
def response = WS.sendRequest(findTestObject('Object Repository/POST - login success'))

// Verifikasi status kode = 200
WS.verifyResponseStatusCode(response, 200)
assertThat(response.getStatusCode()).isEqualTo(200)

// PARSING: Ubah response body jadi objek JSON
def jsonResponse = new JsonSlurper().parseText(response.getResponseBodyContent())

// Verifikasi token ada dan tidak kosong
assertThat(jsonResponse.token).isNotNull()
assertThat(jsonResponse.token).isNotEmpty()

// (Opsional) Cetak token
println &quot;Token diterima: ${jsonResponse.token}&quot;
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
