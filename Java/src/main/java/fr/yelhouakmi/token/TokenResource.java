package fr.yelhouakmi.token;

import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import java.util.UUID;

import javax.ws.rs.Consumes;
import javax.ws.rs.GET;
import javax.ws.rs.POST;
import javax.ws.rs.Path;
import javax.ws.rs.PathParam;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;
import javax.ws.rs.core.Response;
import javax.ws.rs.core.Response.Status;

@Path("/token")
public class TokenResource {

    private static Map<String, String> db = Collections.synchronizedMap(new HashMap<>());
    
    @POST
    @Path("/login")
    @Consumes(MediaType.APPLICATION_JSON)
    @Produces(MediaType.APPLICATION_JSON)
    public Response login(LoginModel data) {
        
        if(data.getPassword().equals("test")){
            var token = UUID.randomUUID().toString();
            db.put(token, data.getUsername());
            var response = new TokenModel(token, data.getUsername());
            return Response.ok(response).build();
        }
        return Response.status(Status.FORBIDDEN).build();
    }

    @GET
    @Path("/check/{token}")
    public Response check(@PathParam("token") String token) {

        if(db.containsKey(token)) {
            var user = db.get(token);
            return Response.ok(user).build();
        }
        return Response.status(Status.UNAUTHORIZED).build();
    }
}
