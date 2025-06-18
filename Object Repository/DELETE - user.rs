<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DELETE - user</name>
   <tag></tag>
   <elementGuidId>93384989-db07-43ba-a59a-b48530196eff</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
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
      <webElementGuid>613aee82-32e7-4f49-85c6-cb6c231cbc7e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-api-key</name>
      <type>Main</type>
      <value>reqres-free-v1</value>
      <webElementGuid>20ef54a5-dc4a-4449-b5d6-99df0a48d13c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.2.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>https://reqres.in/api/users/2</restUrl>
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
import static org.assertj.core.api.Assertions.*

// Kirim request DELETE
def response = WS.sendRequest(findTestObject('Object Repository/DELETE - user'))

// Verifikasi status code adalah 204 (berhasil tanpa content)
WS.verifyResponseStatusCode(response, 204)
assertThat(response.getStatusCode()).isEqualTo(204)

// Tidak perlu body response karena DELETE 204 = tanpa konten
println &quot;User berhasil dihapus, response kosong.&quot;
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
