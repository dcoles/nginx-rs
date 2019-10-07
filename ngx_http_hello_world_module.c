#include <ngx_config.h>
#include <ngx_core.h>
#include <ngx_http.h>

extern ngx_int_t ngx_http_hello_world_handler(ngx_http_request_t *r);
extern ngx_int_t ngx_http_hello_world_access_handler(ngx_http_request_t *r);
static char *ngx_http_hello_world(ngx_conf_t *cf, ngx_command_t *cmd, void *conf);
static ngx_int_t ngx_http_hello_world_init(ngx_conf_t *cf);

static ngx_command_t ngx_http_hello_world_commands[] = {

    { ngx_string("hello_world"),
      NGX_HTTP_LOC_CONF|NGX_CONF_NOARGS,
      ngx_http_hello_world,
      0,
      0,
      NULL },

      ngx_null_command
};

static ngx_http_module_t ngx_http_hello_world_module_ctx = {
    NULL,                                  /* preconfiguration */
    ngx_http_hello_world_init,             /* postconfiguration */

    NULL,                                  /* create main configuration */
    NULL,                                  /* init main configuration */

    NULL,                                  /* create server configuration */
    NULL,                                  /* merge server configuration */

    NULL,                                  /* create location configuration */
    NULL                                   /* merge location configuration */
};

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

static char *ngx_http_hello_world(ngx_conf_t *cf, ngx_command_t *cmd, void *conf)
{
    ngx_http_core_loc_conf_t *clcf;

    clcf = ngx_http_conf_get_module_loc_conf(cf, ngx_http_core_module);

    clcf->handler = ngx_http_hello_world_handler;

    return NGX_CONF_OK;
}

static ngx_int_t ngx_http_hello_world_init(ngx_conf_t *cf)
{
    ngx_http_handler_pt        *h;
    ngx_http_core_main_conf_t  *cmcf;

    cmcf = ngx_http_conf_get_module_main_conf(cf, ngx_http_core_module);

    h = ngx_array_push(&cmcf->phases[NGX_HTTP_ACCESS_PHASE].handlers);
    if (h == NULL) {
        return NGX_ERROR;
    }

    *h = ngx_http_hello_world_access_handler;

    return NGX_OK;
}
