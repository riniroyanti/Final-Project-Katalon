<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create User</name>
   <tag></tag>
   <elementGuidId>05c7b8a7-4cbe-4078-9c7c-91e6e5526f7b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;: \&quot;${nama}\&quot;,\n  \&quot;job\&quot;: \&quot;${pekerjaan}\&quot;,\n  \&quot;salary\&quot;: ${penghasilan}\n}&quot;,
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
      <webElementGuid>dfb0e6a9-b1b9-4f2e-8096-f7d43f614d91</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.3</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Rini'</defaultValue>
      <description></description>
      <id>ee1357b8-290d-4f25-8c7f-8bee9eda9a71</id>
      <masked>false</masked>
      <name>nama</name>
   </variables>
   <variables>
      <defaultValue>'QA'</defaultValue>
      <description></description>
      <id>6ace8489-c57b-4552-ac49-814f4b288ffe</id>
      <masked>false</masked>
      <name>pekerjaan</name>
   </variables>
   <variables>
      <defaultValue>11000000</defaultValue>
      <description></description>
      <id>f98e2950-4756-46f1-ab62-25fe51c3fbcc</id>
      <masked>false</masked>
      <name>penghasilan</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
