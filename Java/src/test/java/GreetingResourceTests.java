import org.junit.jupiter.api.Test;

import fr.yelhouakmi.GreetingResource;
import io.quarkus.test.common.http.TestHTTPEndpoint;
import io.quarkus.test.junit.QuarkusTest;

import static io.restassured.RestAssured.given;
import static org.hamcrest.CoreMatchers.is;

import javax.ws.rs.core.Response.Status;

@QuarkusTest
@TestHTTPEndpoint(GreetingResource.class)
public class GreetingResourceTests {
    
    @Test
    public void shouldSayHello() {
        given()
          .when().get()
          .then()
             .statusCode(Status.OK.getStatusCode())
             .body(is("Hello, World!"));
    }

}
