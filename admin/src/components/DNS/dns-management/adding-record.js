import React, { useState } from "react";
import { Modal, Button, Form, Input, Select, message } from "antd";
import { FiX } from "react-icons/fi";
import axios from "axios";

const { Option } = Select;

const AddRecord = ({ handleCancel, handleOk, records, doid, fetchData }) => {
  //  -------------state -------------
  const [, setLoading] = useState(false);

  //   // ------- token ----------
  const baseUrl = process.env.REACT_APP_API_URL;
  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  //   // ------- apply button ---------

  const handleApply = async (data) => {
    const inputData = {
      subdomain_name: data.subdomain_name,
      address: data.address,
      dns_type: data.dns_type,
      foreign_key: doid,
    };

    await axios
      .post(`${baseUrl}/settings/dns/zone_record/creation`, inputData, {
        headers: {
          "content-type": "application/json",
          ...auth,
        },
      })
      .then((res) => {
        if (res.data.operation_status === "Success") {
          message.success("Successful!");
          fetchData();
          handleOk();
          setLoading(false);
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        console.log(err);
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        message.error("Operation Failed! ");
      });
  };

  const layout = {
    labelCol: {
      span: 8,
    },
    wrapperCol: {
      span: 16,
    },
  };

  return (
    <React.Fragment>
      <Modal
        width={800}
        title={null}
        footer={null}
        closeIcon={<FiX className="close-icon" />}
        visible={records}
        onCancel={handleCancel}
        onOk={handleOk}
      >
        <div className="container-adding-records">
          <Form {...layout} size="large" onFinish={handleApply}>
            <Form.Item
              label="Sub domain name"
              name="subdomain_name"
              rules={[
                { required: true, message: "Please input sub domain name!" },
              ]}
            >
              <Input placeholder="text here ..." size="large" />
            </Form.Item>
            <Form.Item
              label="Adress"
              name="address"
              rules={[{ required: true, message: "Please input the address!" }]}
            >
              <Input placeholder="text here ..." size="large" />
            </Form.Item>
            <Form.Item
              label="Type"
              name="dns_type"
              rules={[
                {
                  required: true,
                  message: "Please select your type!",
                },
              ]}
            >
              <Select
                size="large"
                className="select-option"
                placeholder="Select here ..."
              >
                <Option value="A">A</Option>
                <Option value="AAAA">AAAA</Option>
                <Option value="MX 10">MX 10</Option>
                <Option value="CNAME">CNAME</Option>
                <Option value="PTR">PTR</Option>
                <Option value="CERT">CERT</Option>
                <Option value="SRV">SRV</Option>
                <Option value="TXT">TXT</Option>
                <Option value="SOA">SOA</Option>
              </Select>
            </Form.Item>
            <Form.Item>
              <Button
                className="button-apply4"
                size="large"
                htmlType="submit"
                type="primary"
              >
                Submit
              </Button>
            </Form.Item>
          </Form>
        </div>
      </Modal>
    </React.Fragment>
  );
};

export default AddRecord;
