import { z, ZodType, ZodNullable } from "zod";

const schema = z.object({
  property1: z.nullable(z.string()),
  property2: z.nullable(z.number()),
});

export const parseWithNullDefaults = <T extends ZodType<any>>(
        schema: any,
        data: Record<string, any>,
): ZodNullable<T['_output']> => {
        const parsedData: Record<string, any> = {};

        // Loop through the keys in the schema
        for (const key of Object.keys(schema.shape)) {
                if (data[key] !== undefined) {
                        // If the property is present in the input data, parse it
                        parsedData[key] = schema.shape[key].parse(data[key]);
                } else {
                        // If the property is missing, set it to null
                        parsedData[key] = null;
                }
        }

        return schema.parse(parsedData);
};


const inputData = { property1: "Hello" };
const result = parseWithNullDefaults(schema, inputData);

console.log(result);
