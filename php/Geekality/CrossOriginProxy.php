<?php

namespace Geekality;

/**
 * Cross-origin request proxy for client-side scripts.
 *
 * @see https://github.com/Svish/php-cross-domain-proxy
 */
class CrossOriginProxy
{
    /**
     * Proxies the incoming request and outputs the response, including headers.
     *
     * @param whitelist       Array of request URLs whitelist rules.
     * @param curl_opts       Array of cURL options for proxy request.
     * @param zlib            Value for zlib.output_compression.
     *
     * @see http://php.net/manual/function.curl-setopt.php
     * @see http://php.net/manual/zlib.configuration.php
     */
    public static function proxy(array $whitelist = [], array $curl_opts = [], $zlib = 'On')
    {
        require __DIR__.DIRECTORY_SEPARATOR.'..'.DIRECTORY_SEPARATOR.'proxy.php';
    }
}