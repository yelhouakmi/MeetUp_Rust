package token;

import io.quarkus.test.common.http.TestHTTPEndpoint;
import io.quarkus.test.junit.QuarkusTest;
import io.restassured.http.ContentType;

import static io.restassured.RestAssured.given;
import static org.hamcrest.CoreMatchers.is;

import javax.ws.rs.core.Response.Status;

import org.jboss.resteasy.reactive.RestResponse.StatusCode;

import static org.hamcrest.CoreMatchers.equalTo;

import org.junit.jupiter.api.Test;

import fr.yelhouakmi.token.LoginModel;
import fr.yelhouakmi.token.TokenResource;

@QuarkusTest
@TestHTTPEndpoint(TokenResource.class)
public class TokenResourceTests {

    @Test
    public void shouldLogin() {
        var user = "User";
        var login = new LoginModel();
        login.setUsername(user);
        login.setPassword("test");


        given()
          .body(login)
          .contentType(ContentType.JSON)
          .when().post("/login")
          .then()
             .statusCode(Status.OK.getStatusCode())
             .contentType(ContentType.JSON)
             .body("user", equalTo(user));
    }

    @Test
    public void shouldFailToLogin() {
        var user = "User";
        var login = new LoginModel();
        login.setUsername(user);
        login.setPassword("nope");


        given()
          .body(login)
          .contentType(ContentType.JSON)
          .when().post("/login")
          .then()
             .statusCode(Status.FORBIDDEN.getStatusCode());
    }

    @Test
    public void shouldCheck() {
        var user = "User";
        var login = new LoginModel();
        login.setUsername(user);
        login.setPassword("test");


        String token = given()
          .body(login)
          .contentType(ContentType.JSON)
          .when().post("/login")
          .then()
             .statusCode(Status.OK.getStatusCode())
             .contentType(ContentType.JSON)
             .extract().path("token");


        given()
          .when().get("/check/{token}", token)
          .then()
             .statusCode(Status.OK.getStatusCode());
    }

    @Test
    public void shouldNotCheck() {


        given()
          .when().get("/check/{token}", "nope")
          .then()
             .statusCode(Status.UNAUTHORIZED.getStatusCode());
    }
}
