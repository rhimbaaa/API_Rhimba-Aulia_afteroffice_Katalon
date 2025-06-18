import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper as JsonSlurper
import static org.assertj.core.api.Assertions.*

// Kirim request login gagal (misal password kosong)
def response = WS.sendRequest(findTestObject('POST - login failed'))

// Verifikasi status code adalah 400
WS.verifyResponseStatusCode(response, 400)

assertThat(response.getStatusCode()).isEqualTo(400)

// Parse response JSON
def jsonResponse = new JsonSlurper().parseText(response.getResponseBodyContent())

// Validasi isi error-nya sesuai
assertThat(jsonResponse.error).isEqualTo('Missing password')

println("Pesan error: $jsonResponse.error")

