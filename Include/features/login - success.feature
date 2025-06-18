import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper
import static org.assertj.core.api.Assertions.*

TestObject loginRequest
ResponseObject response

@Given("saya menyiapkan request login dengan email dan password valid")
def prepareLoginRequest() {
    loginRequest = findTestObject('Object Repository/POST - login success')
}

@When("saya mengirim request login")
def sendLoginRequest() {
    response = WS.sendRequest(loginRequest)
}

@Then("saya menerima response 200 dan token yang tidak kosong")
def verifyResponse() {
    WS.verifyResponseStatusCode(response, 200)
    def json = new JsonSlurper().parseText(response.getResponseBodyContent())
    assertThat(json.token).isNotNull()
    assertThat(json.token).isNotEmpty()
}