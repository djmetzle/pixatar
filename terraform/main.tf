provider "aws" {
  region = "us-west-1"
}

resource "aws_s3_bucket" "site_s3_bucket" {
  bucket = "pixatar-djmetzle-io"
}
