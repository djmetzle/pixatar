provider "aws" {
  alias  = "aws-east"
  region = "us-east-1"
}

resource "aws_acm_certificate" "cert" {
  provider          = aws.aws-east
  domain_name       = "pixatar.djmetzle.io"
  validation_method = "DNS"
}
