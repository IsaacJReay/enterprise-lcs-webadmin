import React, { useState } from "react";
import { Button, Form, Input, message } from "antd";
import axios from "axios";

const CreateDomain = () => {
  //   // -----state ---------
  const [, setLoading] = useState(false);
  const [form] = Form.useForm();

  //   // ------- token ----------
  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  //   // ------- apply button ---------

  const handleApply = async (data) => {
    const inputData = {
      domain_name: data.domain_name,
    };
    await axios
      .post(
        "http://10.42.0.188:8080/private/api/settings/dns/domain_name/creation",
        inputData,
        {
          headers: {
            "content-type": "application/json",
            ...auth,
          },
        }
      )

      .then((res) => {
        if (res.data.operation_status === "Success") {
          setTimeout(() => {
            message.success("Successful!");
          }, 1000);
          form.resetFields();
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        message.error(err.response.data.reason);
      });
  };

  return (
    <React.Fragment>
      <Form form={form} layout="inline" onFinish={handleApply}>
        <Form.Item
          label="Domain name"
          name="domain_name"
          rules={[{ required: true, message: "Input domain name!" }]}
        >
          <Input
            placeholder="example.com "
            size="large"
            className="input-info-dns"
          />
        </Form.Item>
        <Form.Item>
          <Button
            type="primary"
            htmlType="submit"
            className="button-update"
            size="large"
          >
            Create
          </Button>
        </Form.Item>
      </Form>
    </React.Fragment>
  );
};

export default CreateDomain;
