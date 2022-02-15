package fr.yelhouakmi.token;

import io.quarkus.runtime.annotations.RegisterForReflection;

@RegisterForReflection
public class TokenModel {
    private String token;
    private String user;

    public TokenModel(String token, String user) {
        this.token = token;
        this.user = user;
    }
    
    public String getToken() {
        return token;
    }
    public void setToken(String token) {
        this.token = token;
    }
    public String getUser() {
        return user;
    }
    public void setUser(String user) {
        this.user = user;
    }

    
}
