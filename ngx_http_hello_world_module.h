#ifndef NGX_HTTP_HELLO_WORLD_MODULE_H_
#define NGX_HTTP_HELLO_WORLD_MODULE_H_
#include <ngx_config.h>
#include <ngx_core.h>
#include <ngx_http.h>

typedef struct {
    ngx_http_complex_value_t  *text;
} ngx_http_hello_world_loc_conf_t;

extern ngx_module_t  ngx_http_hello_world_module;

ngx_int_t ngx_http_hello_world_handler(ngx_http_request_t *r);
ngx_int_t ngx_http_hello_world_access_handler(ngx_http_request_t *r);

#endif // NGX_HTTP_HELLO_WORLD_MODULE_H_
