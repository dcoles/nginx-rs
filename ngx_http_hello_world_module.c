#include "ngx_http_hello_world_module.h"

extern char *ngx_http_hello_world(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);
extern char *ngx_http_hello_world_set_text(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);

static ngx_command_t ngx_http_hello_world_commands[] = {

    { ngx_string("hello_world"),
      NGX_HTTP_LOC_CONF|NGX_CONF_NOARGS,
      ngx_http_hello_world,
      0,
      0,
      NULL },

    { ngx_string("hello_world_text"),
      NGX_HTTP_LOC_CONF|NGX_CONF_TAKE1,
      ngx_http_hello_world_set_text,
      NGX_HTTP_LOC_CONF_OFFSET,
      0,
      NULL },

      ngx_null_command
};

extern ngx_http_module_t ngx_http_hello_world_module_ctx;

ngx_module_t  ngx_http_hello_world_module = {
    NGX_MODULE_V1,
    &ngx_http_hello_world_module_ctx,      /* module context */
    ngx_http_hello_world_commands,         /* module directives */
    NGX_HTTP_MODULE,                       /* module type */
    NULL,                                  /* init master */
    NULL,                                  /* init module */
    NULL,                                  /* init process */
    NULL,                                  /* init thread */
    NULL,                                  /* exit thread */
    NULL,                                  /* exit process */
    NULL,                                  /* exit master */
    NGX_MODULE_V1_PADDING
};
