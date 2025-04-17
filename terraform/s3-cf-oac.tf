resource "aws_s3_bucket_policy" "cf_oac_access" {
  bucket = aws_s3_bucket.site_s3_bucket.id
  policy = data.aws_iam_policy_document.cloudfront_oac_access.json
}

data "aws_iam_policy_document" "cloudfront_oac_access" {
  statement {
    principals {
      type        = "Service"
      identifiers = ["cloudfront.amazonaws.com"]
    }

    actions = [
      "s3:GetObject"
    ]

    resources = [
      aws_s3_bucket.site_s3_bucket.arn,
      "${aws_s3_bucket.site_s3_bucket.arn}/*"
    ]

    condition {
      test     = "StringEquals"
      variable = "AWS:SourceArn"
      values   = [aws_cloudfront_distribution.site_cloudfront.arn]
    }
  }
}
