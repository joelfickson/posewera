package Config

import (
	"github.com/gin-gonic/gin"
	"net/http"
)

func CreateServer() (app *gin.Engine) {
	app = gin.Default()
	app.GET("/ping", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	}).GET("/hello", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "hello",
		})
	})

	return app

}
