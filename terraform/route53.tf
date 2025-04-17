locals {
  domain_zone_id = "ZJJ2LHT3T6L5T"
}

resource "aws_route53_record" "site_record" {
  zone_id = local.domain_zone_id
  name    = "pixatar.djmetzle.io"
  type    = "CNAME"
  ttl     = 3600
  records = [aws_cloudfront_distribution.site_cloudfront.domain_name]
}
