import 'server-only';
import { NextResponse } from "next/server";
import { S3Client, PutObjectCommand } from "@aws-sdk/client-s3";

/**
 * Initialize S3 client with credentials from environment variables
 */
const s3Client = new S3Client({
    region: process.env.AWS_S3_REGION as string,
    credentials: {
        accessKeyId: process.env.AWS_S3_ACCESS_KEY_ID as string,
        secretAccessKey: process.env.AWS_S3_SECRET_ACCESS_KEY as string,
    },
});

/**
 * Uploads a file to S3
 * @param {Buffer} file - The file buffer to upload
 * @param {string} fileName - The name of the file
 * @param {string} contentType - The MIME type of the file
 * @returns {Promise<string>} The uploaded file name
 */
async function uploadFileToS3(file: Buffer, fileName: string, contentType: string): Promise<string> {
    const fileBuffer = file;
   
    const params = {
        Bucket: process.env.AWS_S3_BUCKET_NAME as string,
        Key: `imagesFolder/${fileName}-${Date.now()}`,
        Body: fileBuffer,
        ContentType: contentType,
    };

    const command = new PutObjectCommand(params);
    await s3Client.send(command);
    return fileName;
}

/**
 * API route handler for POST requests
 * @param {Request} req - The incoming request object
 * @returns {Promise<Response>} The response object
 */
export async function POST(req: Request): Promise<Response> {
    try {
        // Parse the incoming form data
        const formData = await req.formData();
        const file = formData.get("file") as File;

        // Validate file presence
        if (!file) {
            return NextResponse.json({ error: "File is required." }, { status: 400 });
        }

        // Check if the file size is under 10MB
        const maxSize = 10 * 1024 * 1024; // 10MB in bytes
        if (file.size > maxSize) {
            return NextResponse.json({ error: "File size must be under 10MB." }, { status: 400 });
        }

        // Check if the file is an image
        const allowedTypes = ['image/jpeg', 'image/jpg', 'image/png', 'image/gif'];
        if (!allowedTypes.includes(file.type)) {
            return NextResponse.json({ error: "File must be an image (jpg, jpeg, png, or gif)." }, { status: 400 });
        }

        // Convert file to buffer for S3 upload
        const buffer = Buffer.from(await file.arrayBuffer());
        // Upload file to S3 and get the filename or URL
        const fileName = await uploadFileToS3(buffer, file.name, file.type);

        // Return success response with the filename
        return NextResponse.json({ success: true, fileName }, { status: 200 });
    } catch (error) {
        // Return error response if any exception occurs
        return NextResponse.json({ error: "Error uploading file" }, { status: 500 });
    }
}